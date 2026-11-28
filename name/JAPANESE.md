# Japanese script, and how it relates to hanzi

Notes for naming things in Japanese, as `name` does for Greek and Hebrew, and
as `item` will need to for EDINET, Japan's EDGAR.

## Three scripts at once

Japanese writes with three scripts in the same sentence:

- **Kanji (漢字)**, literally "Han characters", the same word as Chinese
  *hànzì*. Borrowed from Chinese from about the 5th century on, they carry
  most of the meaning: nouns, verb stems, and nearly all technical vocabulary.
- **Hiragana (ひらがな)**, a phonetic syllabary for grammar: particles, verb
  endings, auxiliaries.
- **Katakana (カタカナ)**, a second syllabary, mostly for foreign loanwords:
  キャッシュ・フロー (*kyasshu furō*, "cash flow").

Accounting vocabulary is almost all kanji compounds, which is why it looks like
Chinese. The giveaway that a sentence is Japanese is the few rounded hiragana
between the kanji:

> 親会社株主**に**帰属**する**当期純利益
> net income attributable to owners of the parent

に is a particle ("to") and する ("to do") turns 帰属 ("belonging") into a verb.

## The same characters, simplified differently

After WWII Japan simplified some characters its own way (*shinjitai*, 新字体);
the PRC later simplified more of them differently; Taiwan and Hong Kong kept the
traditional forms. One word can have three spellings:

| Meaning              | Japanese | Traditional | Simplified |
|----------------------|----------|-------------|------------|
| sell                 | 売       | 賣          | 卖         |
| manage, operate      | 営       | 營          | 营         |
| pass through, steady | 経       | 經          | 经         |
| record               | 録       | 錄          | 录         |
| return, belong       | 帰       | 歸          | 归         |
| issue, emit          | 発       | 發          | 发         |

## Same characters, different words

Pronunciation is unrelated (Japanese readings came from older Chinese, filtered
through Japanese sounds), and meanings have drifted. Some terms from EDINET and
its API key page:

| Japanese    | Reading            | Meaning in Japanese          | Read as Chinese (pinyin) | In Chinese                                  |
|-------------|--------------------|------------------------------|--------------------------|---------------------------------------------|
| 利益        | rieki              | profit                       | lìyì                     | profit, benefit (same)                      |
| 売上高      | uriagedaka         | net sales                    | mài shàng gāo            | not a word; Chinese says 营业额 or 销售额     |
| 営業利益    | eigyō rieki        | operating income             | yíngyè lìyì              | operating profit (same)                     |
| 経常利益    | keijō rieki        | ordinary income              | jīngcháng lìyì           | 经常 means "often": no such concept         |
| 親会社      | oyagaisha          | parent company               | qīn huìshè               | 会社 is Japanese; Chinese says 母公司        |
| 株主        | kabunushi          | shareholder                  | zhū zhǔ                  | Chinese says 股东 (gǔdōng)                   |
| 当期純利益  | tōki jun rieki     | net income for the period    | dāngqī chún lìyì         | understandable; Chinese says 本期净利润      |
| 発行        | hakkō              | issue (a key, shares)        | fāxíng                   | issue, publish (same)                       |
| 画面        | gamen              | (computer) screen            | huàmiàn                  | picture, frame; a screen is 屏幕            |
| 連絡先      | renrakusaki        | contact details              | lián luò xiān            | not a word; Chinese says 联系方式            |
| 登録        | tōroku             | register                     | dēng lù                  | 登录 is "log in"                            |
| 氏名        | shimei             | full name                    | shìmíng                  | full name (formal, same)                    |
| 電話番号    | denwa bangō        | phone number                 | diànhuà fānhào           | Chinese says 电话号码 (hàomǎ)               |
| 必須        | hissu              | required                     | bìxū                     | must, required (same)                       |

## What this means for `name` and `item`

- An entity can have a Japanese name, a Chinese one, and English ones, as
  `name` already gives Ἀλέξανδρος the aliases "Alexander" and "Alessandro".
  Identical-looking characters in Japanese and Chinese are still separate
  aliases: they can name different concepts.
- Some Japanese concepts have no English or US GAAP counterpart. 経常利益
  (*keijō rieki*, "ordinary income": operating income plus non-operating gains
  and losses, before extraordinary items) is Japan's headline profit figure; it
  wants its own item rather than an approximate American one.
- Words that both translate as "revenue" are not one concept: 売上高 (net sales)
  for most companies, 営業収益 (*eigyō shūeki*, operating revenue) for banks,
  brokers and utilities, whose scope differs.
- EDINET's XBRL element names are English (`jppfs_cor:NetSales`); the labels
  readers see are Japanese. Company extension elements may only be
  identifiable by their Japanese labels, as TSLA's `tsla_` tags are by theirs.
