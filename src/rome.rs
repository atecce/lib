use crate::name::Name::Romulus;
use crate::name::Name::Remus;
use crate::name::Name::Caesar;
use crate::name::Name::Cicero;
use crate::daemon::Daemon;

pub const ROMULUS: &Daemon = &Daemon {
    names: &[
        Romulus,
    ],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,
};

pub const REMUS: &Daemon = &Daemon {
    names: &[
        Remus,
    ],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None
};

pub const CICERO: &Daemon = &Daemon {
    names: &[
        Cicero,
    ],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,
};

pub const CAESAR: &Daemon = &Daemon {
    names: &[
        Caesar,
    ],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,
};

// // Julio-Claudian dynasty
// Causar Augustus, "Augustus"; 16 January 27 BC - 19 August AD 14
// Tiberius Caesar Augustus, "Tiberius"; 17 September 14 - 16 March 37
// Gaius Caesar Augustus Germanicus, "Caligula"; 18 March 37 - 24 January 41
// Tiberius Claudius Caesar Augustus Germanicus, "Claudius"; 24 January 31 - 13 October 54
// Nero Claudius Caesar Augustus Germanicus, "Nero"; 13 October 54 - 9 June 68
// // Year of the Four Emperors
// Servius Galba Caesar Augustus, "Galba"; 8 June 68 - 15 January 69
// Marcus Otho Caesar Augustus, "Otho", 15 January - 16 April 69
// Aulus Vitellius Germanicus Augustus, "Vitellius"; 19 April - 20 December 69
// // Flavian dynasty
// Caesar Vespasianus Augustus, "Vespasian"; 1 July 69 - 23 June 79
// Titus Caesar Vespasianus Augustus, "Titus"; 24 July 79 - 13 September 81
// Caesar Domitianus Augustus, "Domitian"; 14 September 81 - 18 September 96
// // Nerva-Antonine dynasty
// Nerva Caesar Augustus, "Nerva"; 18 September 96 - 27 January 98
// Caesar Nerva Traianus Augustus, "Trajan"; 28 January 98 - 9 August (?) 117
// Caesar Traianus Hadrianus Augustus, "Hadrian"; 11 August 117 - 10 July 138
// Titus Aelius Hadrianus Antoninus Pius, "Antoninus Pius"; 10 July 138 - 7 March 161
// Marcus Aurelius Antoninus, "Marcus Aurelius"; 7 March 161 - 17 March 180
// Lucius Aurelius Verus, "Lucius Verus"; 7 March 161 - January/February 169
// Marcus Aurelius Commodus Antoninus / Lucius Aelius Aurelius Commodus, "Commodus"; 17 March 180 - 31 December 192
// // Year of the Five Emperors
// Publius Helvius Pertinax, "Pertinax"; 1 January - 28 March 193
// Marcus Didius Severus Julianus, "Didius Julianus"; 28 March - 1 June 193
// // Severan dynasty
// Lucius Septimius Severus Pertinax, "Septimius Severus"; 9 April 193 - 4 February 211
// Publius Septimius Geta, "Geta"; 4 February 211 - 26 December 211
// Marcus Aurelius Antoninus, "Caracalla"; 4 February 211 - 8 April 217
// Marcus Opelius Severus Macrinus, "Macrinus"; 11 April 217 - 8 June 218
// Marcus Opelius Antoninus Diadumenianus, "Diadumenian"; Late May - June 218
// Marcus Aurelius Antoninus, "Elegabalus"; 16 May 218 - 13 March 222
// Marcus Aurelius Severus Alexander, "Severus Alexander"; 14 March 222 - March 235
// // Crisis of the Third Century
// Gaius Julius Verus Maximinus, "Maximinus I (Thrax)"; March 235 - June 238
// Marcus Antonius Gordianus Sempronianus Romanus, "Gordian I"; April - May 238
// Marcus Antonius Gordianus Sempronianus Romanus, "Gordian II"; April May 238
// Marcus Clodius Pupienus Maximus, "Pupienus"; May - August 238
// Decimus Caelius Calvinus Balbinus, "Balbinus"; May - August 238
// Marcus Antonius Gordianus, "Gordian III"; August 238 - February 244
// Marcus Julius Philippus, "Philip I (the Arab)"; February 244 - September/October 249
// Marcus Julius Severus Philippus, "Philip II (the Younger)"; July/August 247 - September/October 249
// Gaius Messius Quintus Traianus Decius, "Decius"; September/October 249 - June 251
// Quintus Herennius Etruscus Messius Decius, "Herennius Etruscus"; May/June - June 251
// Gaius Vibius Trebonianus Gallus, "Trebonianus Gallus"; June 251 - August 253
// Gaius Valens Hostilianus Messius Quintus, "Hostilian"; June - July 251
// Gaius Vibius Afinius Gallus Veldumnianus Volusianus, "Volusianus"; August 251 - August 253
// Marcus Aemilius Aemilianus, "Aemilianus"; July - September 253
// Mar. Silbannacus, "Silbannacus"; September/October 253
// Publius Licinius Valerianus, "Valerian"; September 253 - June 260
// Publius Licinius Egnatius Gallienus, "Gallienus"; September 253 - September 268
// Publius Licinius Cornelius Saloninus Valerianus, "Saloninus", Autumn 260
// Marcus Aurelius Claudius, "Claudius II (Gothicus)"; September 268 - August 270
// Marcus Aurelius Claudius Quintillus, "Quintillus"; August - September 270
// Lucius Domitius Aurelianus, "Aurelian"; August 270 - November 275
// Marcus Claudius Tacitus, "Tacitus"; December 275 - June 276
// Marcus Annius Florianus, "Florianus"; June - September 276
// Marcus Aurelius Probus, "Probus"; June 276 - September 282
// Marcus Aurelius Carus, "Carus"; September 282 - July/August 283
// Marcus Aurelius Carinus, "Carinus"; Spring 283 - August/September 285
// Marcus Aurelius Numerianus, "Numerian"; July/August 283 - November 284
// // // Dominate
// // Tetrarchy
// Gaius Aurelius Valerius Diocletianus, "Diocletian (Jovius)"; 20 November 284 - 1 May 305
// Marcus Aurelius Valerius Maximianus, "Maximian"; 1 April 286 - 1 May 305
// Gaius Galerius Valerius Maximianus, "Galerius"; 1 May 305 - May 311
// Marcus Flavius Valerius Constantius, "Constantius I (Chlorus)"; 1 May 305 - 25 July 306
// Flavius Valerius Severus, "Severus II"; August 306 - March/April 307
// Lucius Domitius Alexander, "Alexander"; 308 - 310
// Marcus Aurelius Valerius Maxentius, "Maxentius"; 28 October 306 - 28 October 312
// Valerius Licinianus Licinius, "Licinius"; 11 November 308 - 19 September 324
// Galerius Valerius Maximinus, "Maximinus II (Daza)"; 310 - July 313
// Aurelius Valerius Valens, "Valerius Valens"; October 316 - January 317
// Mar. Martinianus, "Martinian"; July - 19 September 324 ;
// // Constantinian dynasty
// Flavius Valerius Constantinus, "Constantine I (the Great)"; 25 July 306 - 22 May 337
// Falvius Claudius Constantinus, "Constantine II"; 9 September 337 - April 340
// Flavius Julius Constans, "Constans I"; 9 September 337 - January 350
// Flavius Julius Constantius, "Constantius II"; 9 September 337 - 3 November 361
// Magnus Magnentius, "Magnentius"; 18 January 350 - 10 August 353
// "Vetranio"; 1 March - 25 December 350
// Julius Nepotianus, "Nepotianus"; 3 June - 30 June 350
// Flavius Claudius Julianus, "Julian (the Apostate)"; 3 November 361 - 26 June 363
// Jovianus, "Jovian"; 27 June 363 - 17 February 364
