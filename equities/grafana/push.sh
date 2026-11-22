#!/bin/zsh
# Push dashboards to Grafana, replacing each by its uid (so links and
# bookmarks keep working).
#
#     equities/grafana/push.sh             every *.json beside this script
#     equities/grafana/push.sh auto.json   just these
#
# Needs a service account token with the Editor role in GRAFANA_TOKEN
# (Administration > Users and access > Service accounts). GRAFANA_URL
# defaults to the local Homebrew Grafana.
source ~/.profile
url=${GRAFANA_URL:-http://localhost:3000}
[ -n "$GRAFANA_TOKEN" ] || { echo "GRAFANA_TOKEN is not set" >&2; exit 1; }
dir=${0:A:h}
files=("$@")
[ ${#files} -eq 0 ] && files=($dir/*.json)
failed=0
for f in $files; do
  [ -f $f ] || f=$dir/$f
  # Wrap the dashboard for the API; drop its numeric id, which belongs to
  # whichever Grafana it was exported from. The uid identifies it.
  body=$(python3 -c '
import json, sys
d = json.load(open(sys.argv[1]))
d.pop("id", None)
print(json.dumps({"dashboard": d, "overwrite": True, "message": "pushed from " + sys.argv[1].split("/")[-1]}))
' $f) || { failed=1; continue; }
  res=$(curl -s -X POST $url/api/dashboards/db \
    -H "Authorization: Bearer $GRAFANA_TOKEN" -H "Content-Type: application/json" \
    --data-binary "$body")
  python3 -c '
import json, sys
r = json.loads(sys.argv[1])
if r.get("status") == "success":
    print("%s: version %s, %s%s" % (sys.argv[2], r["version"], sys.argv[3], r["url"]))
else:
    print("%s: %s" % (sys.argv[2], r.get("message", r)), file=sys.stderr); sys.exit(1)
' "$res" ${f:t} $url || failed=1
done
exit $failed
