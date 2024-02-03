pub(crate) mod button_icon;
pub mod button_wrapper;
pub mod filtered_tag_button;
pub mod icon_button;

#[derive(Default)]
pub enum ButtonSize {
    XS,
    SM,
    #[default]
    MD,
    LG,
}

impl std::str::FromStr for ButtonSize {
    type Err = ();

    fn from_str(input: &str) -> Result<ButtonSize, Self::Err> {
        match input {
            "xs" => Ok(ButtonSize::XS),
            "sm" => Ok(ButtonSize::SM),
            "md" => Ok(ButtonSize::MD),
            "lg" => Ok(ButtonSize::LG),

            _ => Ok(Default::default()),
        }
    }
}

impl ButtonSize {
    pub fn from_str_with_default(input: &str) -> Self {
        input
            .parse::<Self>()
            .expect("to have default handled for ButtonSize!")
    }
}

impl std::fmt::Display for ButtonSize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let printable = match self {
            ButtonSize::XS => "text-xs min-w-[1.5rem] h-6  px-2",
            ButtonSize::SM => "text-sm min-w-[2rem]   h-8  px-3",
            ButtonSize::MD => "text-md min-w-[2.5rem] h-10 px-4",
            ButtonSize::LG => "text-lg min-w-[3rem]   h-12 px-6",
        };
        write!(f, "{}", printable)
    }
}

#[derive(Default, PartialEq, Eq)]
pub enum ButtonVarient {
    #[default]
    SOLID,
    OUTLINE,
    GHOST,
    LINK,
}

impl std::fmt::Display for ButtonVarient {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let printable = match self {
            ButtonVarient::SOLID => "solid",
            ButtonVarient::OUTLINE => "outline",
            ButtonVarient::GHOST => "ghost",
            ButtonVarient::LINK => "link",
        };
        write!(f, "{}", printable)
    }
}

#[derive(Default)]
pub enum ButtonColor {
    #[default]
    GRAY,
    GREEN,
    BLUE,
    RED,
}

impl std::fmt::Display for ButtonColor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let printable = match self {
            ButtonColor::GRAY => "gray",
            ButtonColor::GREEN => "green",
            ButtonColor::BLUE => "blue",
            ButtonColor::RED => "red",
        };
        write!(f, "{}", printable)
    }
}

#[derive(Default)]
pub enum ButtonStyle {
    #[default]
    SolidGray,
    SolidBlue,
    SolidGreen,
    SolidRed,

    GhostGray,
    GhostBlue,
    GhostGreen,
    GhostRed,

    OutlineGray,
    OutlineBlue,
    OutlineGreen,
    OutlineRed,

    LinkGray,
    LinkBlue,
    LinkGreen,
    LinkRed,
}

impl std::str::FromStr for ButtonStyle {
    type Err = ();

    fn from_str(input: &str) -> Result<ButtonStyle, Self::Err> {
        match input {
            "solid_gray" => Ok(ButtonStyle::SolidGray),
            "solid_blue" => Ok(ButtonStyle::SolidBlue),
            "solid_green" => Ok(ButtonStyle::SolidGreen),
            "solid_red" => Ok(ButtonStyle::SolidRed),

            "ghost_gray" => Ok(ButtonStyle::GhostGray),
            "ghost_blue" => Ok(ButtonStyle::GhostBlue),
            "ghost_green" => Ok(ButtonStyle::GhostGreen),
            "ghost_red" => Ok(ButtonStyle::GhostRed),

            "outline_gray" => Ok(ButtonStyle::OutlineGray),
            "outline_blue" => Ok(ButtonStyle::OutlineBlue),
            "outline_green" => Ok(ButtonStyle::OutlineGreen),
            "outline_red" => Ok(ButtonStyle::OutlineRed),

            "link_gray" => Ok(ButtonStyle::LinkGray),
            "link_blue" => Ok(ButtonStyle::LinkBlue),
            "link_green" => Ok(ButtonStyle::LinkGreen),
            "link_red" => Ok(ButtonStyle::LinkRed),

            _ => Ok(Default::default()),
        }
    }
}

impl ButtonStyle {
    pub fn from_varient_color(varient: &ButtonVarient, color: &ButtonColor) -> Self {
        format!("{varient}_{color}")
            .parse::<Self>()
            .expect("to have all varient and color combinations!")
    }

    pub fn from_str_with_default(input: &str) -> Self {
        input
            .parse::<Self>()
            .expect("to have default handled for ButtonStyle!")
    }
}

impl std::fmt::Display for ButtonStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let printable = match self {
            ButtonStyle::SolidGray => {
                "
                bg-gray-100                   dark:bg-white/20
                text-gray-800                 dark:text-white/90
                hover:bg-gray-200             dark:hover:bg-white/30
                hover:disabled:bg-gray-100    dark:hover:disabled:bg-white/20
                active:bg-gray-300            dark:active:bg-white/40
            "
            }
            ButtonStyle::SolidBlue => {
                "
                bg-blue-500                   dark:bg-blue-200
                text-white                    dark:text-gray-800
                hover:bg-blue-600             dark:hover:bg-blue-300
                hover:disabled:bg-blue-500    dark:hover:disabled:bg-blue-200
                active:bg-blue-700            dark:active:bg-blue-400
            "
            }
            ButtonStyle::SolidGreen => {
                "
                bg-emerald-500                dark:bg-emerald-200
                text-white                    dark:text-gray-800
                hover:bg-emerald-600          dark:hover:bg-emerald-300
                hover:disabled:bg-emerald-500 dark:hover:disabled:bg-emerald-200
                active:bg-emerald-700         dark:active:bg-emerald-400
            "
            }
            ButtonStyle::SolidRed => {
                "
                bg-red-500                    dark:bg-red-200
                text-white                    dark:text-gray-800
                hover:bg-red-600              dark:hover:bg-red-300
                hover:disabled:bg-red-500     dark:hover:disabled:bg-red-200
                active:bg-red-700             dark:active:bg-red-400
            "
            }

            ButtonStyle::GhostGray => {
                "
                text-gray-800                 dark:text-white/90
                hover:bg-gray-100             dark:hover:bg-gray-800
                active:bg-gray-200            dark:active:bg-gray-700
            "
            }
            ButtonStyle::GhostBlue => {
                "
                text-blue-600                 dark:text-blue-200
                bg-transparent
                hover:bg-blue-50              dark:hover:bg-blue-200/10
                active:bg-blue-100            dark:active:bg-blue-200/20
            "
            }
            ButtonStyle::GhostGreen => {
                "
                text-emerald-600              dark:text-emerald-200
                bg-transparent
                hover:bg-emerald-50           dark:hover:bg-emerald-200/10
                active:bg-emerald-100         dark:active:bg-emerald-200/20
            "
            }
            ButtonStyle::GhostRed => {
                "
                text-red-600                  dark:text-red-200
                bg-transparent
                hover:bg-red-50               dark:hover:bg-red-200/10
                active:bg-red-100             dark:active:bg-red-200/20
            "
            }

            ButtonStyle::OutlineGray => {
                "
                border
                border-gray-200               dark:border-white/30
            "
            }
            ButtonStyle::OutlineBlue => {
                "
                border
                border-current
            "
            }
            ButtonStyle::OutlineGreen => {
                "
                border
                border-current
            "
            }
            ButtonStyle::OutlineRed => {
                "
                border
                border-current
            "
            }

            ButtonStyle::LinkGray => {
                "
                p-0 h-auto leading-normal align-baseline
                text-gray-500             dark:text-gray-200
                active:text-gray-700      dark:active:text-gray-500
                hover:underline
                hover:disabled:no-underline
            "
            }
            ButtonStyle::LinkBlue => {
                "
                p-0 h-auto leading-normal align-baseline
                text-blue-500             dark:text-blue-200
                active:text-blue-700      dark:active:text-blue-500
                hover:underline
                hover:disabled:no-underline
            "
            }
            ButtonStyle::LinkGreen => {
                "
                p-0 h-auto leading-normal align-baseline
                text-emerald-500          dark:text-emerald-200
                active:text-emerald-700   dark:active:text-emerald-500
                hover:underline
                hover:disabled:no-underline
            "
            }
            ButtonStyle::LinkRed => {
                "
                p-0 h-auto leading-normal align-baseline
                text-red-500              dark:text-red-200
                active:text-red-700       dark:active:text-red-500
                hover:underline
                hover:disabled:no-underline
            "
            }
        };
        write!(f, "{}", printable)
    }
}
