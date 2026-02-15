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

    predecessor: None,
};

pub const REMUS: &Daemon = &Daemon {
    names: &[
        Remus,
    ],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: None,
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

    predecessor: None,
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

    predecessor: None,
};

// // Julio-Claudian dynasty
// Causar Augustus, "Augustus"; 16 January 27 BC - 19 August AD 14
pub const AUGUSTUS: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: Some(CAESAR),
};

// Tiberius Caesar Augustus, "Tiberius"; 17 September 14 - 16 March 37
pub const TIBERIUS: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: Some(AUGUSTUS),
};

// Gaius Caesar Augustus Germanicus, "Caligula"; 18 March 37 - 24 January 41
pub const CALIGULA: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: Some(TIBERIUS),
};

// Tiberius Claudius Caesar Augustus Germanicus, "Claudius"; 24 January 31 - 13 October 54
pub const CLAUDIUS: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: Some(CALIGULA),
};

// Nero Claudius Caesar Augustus Germanicus, "Nero"; 13 October 54 - 9 June 68
pub const NERO: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: Some(CLAUDIUS),
};

// // Year of the Four Emperors
// Servius Galba Caesar Augustus, "Galba"; 8 June 68 - 15 January 69
pub const GALBA: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: Some(NERO),
};

// Marcus Otho Caesar Augustus, "Otho", 15 January - 16 April 69
pub const OTHO: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: Some(GALBA),
};

// Aulus Vitellius Germanicus Augustus, "Vitellius"; 19 April - 20 December 69
pub const VITELLIUS: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: Some(OTHO),
};

// // Flavian dynasty
// Caesar Vespasianus Augustus, "Vespasian"; 1 July 69 - 23 June 79
pub const VESPASIAN: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: Some(VITELLIUS),
};

// Caesar Domitianus Augustus, "Domitian"; 14 September 81 - 18 September 96
pub const DOMITIAN: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: Some(VESPASIAN),
};

// // Nerva-Antonine dynasty
// Nerva Caesar Augustus, "Nerva"; 18 September 96 - 27 January 98
pub const NERVA: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: Some(DOMITIAN),
};

// Caesar Nerva Traianus Augustus, "Trajan"; 28 January 98 - 9 August (?) 117
pub const TRAJAN: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: Some(NERVA),
};

// Caesar Traianus Hadrianus Augustus, "Hadrian"; 11 August 117 - 10 July 138
pub const HADRIAN: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: Some(TRAJAN),
};

// Titus Aelius Hadrianus Antoninus Pius, "Antoninus Pius"; 10 July 138 - 7 March 161
pub const ANTONINUS_PIUS: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: Some(HADRIAN),
};

// Marcus Aurelius Antoninus, "Marcus Aurelius"; 7 March 161 - 17 March 180
pub const MARCUS_AURELIUS: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: Some(ANTONINUS_PIUS),
};

// Lucius Aurelius Verus, "Lucius Verus"; 7 March 161 - January/February 169
pub const LUCIUS_VERUS: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    // TODO(atec): multiple emperors?
    predecessor: Some(MARCUS_AURELIUS),
};

// Marcus Aurelius Commodus Antoninus / Lucius Aelius Aurelius Commodus, "Commodus"; 17 March 180 - 31 December 192
pub const COMMODUS: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: Some(MARCUS_AURELIUS),
};

// // Year of the Five Emperors
// Publius Helvius Pertinax, "Pertinax"; 1 January - 28 March 193
pub const PERTINAX: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: Some(COMMODUS),
};

// Marcus Didius Severus Julianus, "Didius Julianus"; 28 March - 1 June 193
pub const DIDIUS_JULIANUS: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: Some(PERTINAX),
};

// // Severan dynasty
// Lucius Septimius Severus Pertinax, "Septimius Severus"; 9 April 193 - 4 February 211
pub const SEPTIMIUS_SEVERUS: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: Some(DIDIUS_JULIANUS),
};

// Publius Septimius Geta, "Geta"; 4 February 211 - 26 December 211
pub const GETA: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: Some(SEPTIMIUS_SEVERUS),
};

// Marcus Aurelius Antoninus, "Caracalla"; 4 February 211 - 8 April 217
pub const CARACALLA: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: Some(GETA),
};

// Marcus Opelius Severus Macrinus, "Macrinus"; 11 April 217 - 8 June 218
pub const MACRINUS: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: Some(CARACALLA),
};

// Marcus Opelius Antoninus Diadumenianus, "Diadumenian"; Late May - June 218
pub const DIADUMENIAN: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: Some(MACRINUS),
};

// Marcus Aurelius Antoninus, "Elegabalus"; 16 May 218 - 13 March 222
pub const ELEGABALUS: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: Some(DIADUMENIAN),
};

// Marcus Aurelius Severus Alexander, "Severus Alexander"; 14 March 222 - March 235
pub const SEVERUS_ALEXANDER: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: Some(ELEGABALUS),
};

// // Crisis of the Third Century
// Gaius Julius Verus Maximinus, "Maximinus I (Thrax)"; March 235 - June 238
pub const MAXIMUS_I_THRAX: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: Some(SEVERUS_ALEXANDER),
};

// Marcus Antonius Gordianus Sempronianus Romanus, "Gordian I"; April - May 238
pub const GORDIAN_I: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    predecessor: Some(MAXIMUS_I_THRAX),
};

// Marcus Antonius Gordianus Sempronianus Romanus, "Gordian II"; April - May 238
pub const GORDIAN_II: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    // TODO(atec): multiple emperors
    predecessor: Some(MAXIMUS_I_THRAX),
};

// Marcus Clodius Pupienus Maximus, "Pupienus"; May - August 238
pub const PUPIENUS: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    // TODO(atec): multiple emperors
    predecessor: Some(GORDIAN_I),
};

// Decimus Caelius Calvinus Balbinus, "Balbinus"; May - August 238
pub const BALBINUS: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    // TODO(atec): multiple emperors
    predecessor: Some(GORDIAN_I),
};

// Marcus Antonius Gordianus, "Gordian III"; August 238 - February 244
pub const GORDIAN_III: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    // TODO(atec): multiple emperors
    predecessor: Some(BALBINUS),
};

// Marcus Julius Philippus, "Philip I (the Arab)"; February 244 - September/October 249
pub const PHILIP_I_THE_ARAB: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    // TODO(atec): multiple emperors
    predecessor: Some(GORDIAN_III),
};

// Marcus Julius Severus Philippus, "Philip II (the Younger)"; July/August 247 - September/October 249
pub const PHILIP_II_THE_YOUNGER: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    // TODO(atec): multiple emperors
    predecessor: Some(GORDIAN_III),
};

// Gaius Messius Quintus Traianus Decius, "Decius"; September/October 249 - June 251
pub const DECIUS: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    // TODO(atec): multiple emperors
    predecessor: Some(PHILIP_I_THE_ARAB),
};

// Quintus Herennius Etruscus Messius Decius, "Herennius Etruscus"; May/June - June 251
pub const HERENNIUS_ETRUSCUS: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    // TODO(atec): multiple emperors
    predecessor: Some(PHILIP_I_THE_ARAB),
};

// Gaius Vibius Trebonianus Gallus, "Trebonianus Gallus"; June 251 - August 253
pub const TREBONIANUS_GALLUS: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    // TODO(atec): multiple emperors
    predecessor: Some(DECIUS),
};

// Gaius Valens Hostilianus Messius Quintus, "Hostilian"; June - July 251
pub const HOSTILIAN: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    // TODO(atec): multiple emperors
    predecessor: Some(DECIUS),
};

// Gaius Vibius Afinius Gallus Veldumnianus Volusianus, "Volusianus"; August 251 - August 253
pub const VOLUSIANUS: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    // TODO(atec): multiple emperors
    predecessor: Some(DECIUS),
};

// Marcus Aemilius Aemilianus, "Aemilianus"; July - September 253
pub const AEMILIANUS: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    // TODO(atec): multiple emperors
    predecessor: Some(VOLUSIANUS),
};

// Mar. Silbannacus, "Silbannacus"; September/October 253
pub const SILBANNACUS: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    // TODO(atec): multiple emperors
    predecessor: Some(AEMILIANUS),
};

// Publius Licinius Valerianus, "Valerian"; September 253 - June 260
pub const VALERIAN: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    // TODO(atec): multiple emperors
    predecessor: Some(SILBANNACUS),
};

// Publius Licinius Egnatius Gallienus, "Gallienus"; September 253 - September 268
pub const GALLIENUS: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    // TODO(atec): multiple emperors
    predecessor: Some(SILBANNACUS),
};

// Publius Licinius Cornelius Saloninus Valerianus, "Saloninus", Autumn 260
pub const SALONINUS: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    // TODO(atec): multiple emperors
    predecessor: Some(VALERIAN),
};

// Marcus Aurelius Claudius, "Claudius II (Gothicus)"; September 268 - August 270
pub const CLAUDIUS_II_GOTHICUS: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    // TODO(atec): multiple emperors
    predecessor: Some(GALLIENUS),
};

// Marcus Aurelius Claudius Quintillus, "Quintillus"; August - September 270
pub const QUINTILLUS: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    // TODO(atec): multiple emperors
    predecessor: Some(CLAUDIUS_II_GOTHICUS),
};

// Lucius Domitius Aurelianus, "Aurelian"; August 270 - November 275
pub const AURELIAN: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    // TODO(atec): multiple emperors
    predecessor: Some(QUINTILLUS),
};

// Marcus Claudius Tacitus, "Tacitus"; December 275 - June 276
pub const TACITUS: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    // TODO(atec): multiple emperors
    predecessor: Some(AURELIAN),
};

// Marcus Annius Florianus, "Florianus"; June - September 276
pub const FLORIANUS: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    // TODO(atec): multiple emperors
    predecessor: Some(TACITUS),
};

// Marcus Aurelius Probus, "Probus"; June 276 - September 282
pub const PROBUS: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    // TODO(atec): multiple emperors
    predecessor: Some(FLORIANUS),
};

// Marcus Aurelius Carus, "Carus"; September 282 - July/August 283
pub const CARUS: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    // TODO(atec): multiple emperors
    predecessor: Some(PROBUS),
};

// Marcus Aurelius Carinus, "Carinus"; Spring 283 - August/September 285
pub const CARINUS: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    // TODO(atec): multiple emperors
    predecessor: Some(CARUS),
};

// Marcus Aurelius Numerianus, "Numerian"; July/August 283 - November 284
pub const NUMERIAN: &Daemon = &Daemon {
    names: &[],
    words: &[],
    deeds: &[],

    father: None,
    mother: None,
    teacher: None,

    // TODO(atec): multiple emperors
    predecessor: Some(CARUS),
};
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
// Valentinianus, "Valentinan I (the Great)"; 25/26 February 364 - 17 November 375
// "Valens"; 28 March 364 - 9 August 378
// "Procopius"; 28 September 365 - 27 May 366
// Gratianus, "Gratian"; 17 November 375 - 25 August 383
// "Magnus Maximus"; 25 August 383 - 28 August 388
// "Victor"; 383/387 - 388
// Valentinianus, "Valentinian II"; 28 August 388 - 15 May 392
// "Eugenius"; 22 August 392 - 6 September 394
