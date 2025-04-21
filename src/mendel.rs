// 1. The difference in the form of the ripe seeds. These are either
// spherical or somewhat rounded, and the depressions, if
// any, occur on the surface, and are only shallow; or they are
// irregularly angular and deeply wrinkled (P. quadratum).
pub enum RipeSeedForm {
    Spherical,
    Wrinkled,
}

// 2. The difference in the colour of the seed albumen (endo-
//     sperm). The albumen of the ripe seeds is pale yellow, bright
// yellow, or orange coloured; or it possesses a more or less
// intensive green colour. This difference in colour is obvious
// to see in the seeds, since their coats are translucent.
pub enum SeedAlbumenColor {
    PaleYellow,
    BrightYellow,
    Orange,
    IntensiveGreen
}

// 3. The difference in the colour of the seed coat. This is either
// coloured white, a character consistently associated with
// white flower colour, or it is grey, grey-brown, or leather
// brown with or without violet spots, in which case the colour
// of the standard petal appears violet, that of the wings purple,
// and the stem at the base of the leaf axils is tinged reddish.
// The grey seed coats turn blackish brown in boiling water.
pub enum SeedCoatColor {
    White,
    Grey,
    GreyBrown,
    // TODO(atec): maybe split into two
    LeatherBrownWithOrWithoutVioletSpots,
}

// 4. The difference in the form of the ripe pod. This is either simply
// inflated, never pinched in places, or deeply constricted be-
// tween the seeds and more or less wrinkled (P. saccharatum).
pub enum RipePodForm {
    Inflated,
    Wrinkled,
}

// 5. The difference in the colour of the unripe pod. It is either
// light to dark green or coloured a bright yellow, a colour
// shared by stems, leaf veins, and sepals.
// (One variety has a beautiful brown-red pod colour that transforms into violet and blue
// at the time of ripening. The experiment with this character was begun only during the
// past year.)
pub enum UnripePodColor {
    LightToDarkGreen,
    BrightYellow,
    // TODO(atec): maybe remove
    BrownRed,
}

// 6. The difference in the placement of the flowers. They are
// either axial, i.e., distributed along the stem, or terminal,
// accumulated at the end of the stem in a short false umbel,
// in which case the upper part of the stem is more or less
// widened in cross-section (P. umbellatum).
pub enum FlowerPlacement {
    Axial,
    Terminal,
}

// 7. The difference in the length of the stem. The length of the
// stem is very different in individual forms; however, for
// each one it is a constant character undergoing insignifi-
// cant changes insofar as the plants are healthy and are
// raised in the same soil. In the experiments with this char-
// acter, to obtain a confident difference, the long stem of 6–7
// feet was united with the short one of 0.75–1.5 feet.
pub enum StemLength {
    SixToSevenFeet,
    PointSevenFiveToOnePointFiveFeet,
}

// 1st experiment 60 fertilisations were performed on 15 plants
// 2nd “ 58“ “ “ 10“
// 3rd “ 35“ “ “ 10“
// 4th “ 40“ “ “ 10“
// 5th “ 23“ “ “ 5“
// 6th “ 34“ “ “ 10“
// 7th “ 37“ “ “ 10“
