pub fn test_option()->Option<u8> {
    let mut option_1 :Option<u8> = None;
    option_1 = Some(5);
    return option_1;
}

pub fn test_option_string()->Option<String> {
    let mut option_1 :Option<String> = None;
    option_1 = Some("Option Enum with Type String".to_string());
    return option_1;
}


pub enum CustomCharacterEnum {
    None,
    Archer,
    Warrior
}

pub fn test_custom_option_enum()->CustomCharacterEnum {
    let mut option_1 :CustomCharacterEnum = CustomCharacterEnum::None;
    option_1 = CustomCharacterEnum::Archer;
    return option_1;
}

impl ToString for CustomCharacterEnum {
    fn to_string(&self)->String {
        match self {
            CustomCharacterEnum::None => "None".to_string(),
            CustomCharacterEnum::Archer => "Archer".to_string(),
            CustomCharacterEnum::Warrior => "Warrior".to_string(),
        }
    }
}