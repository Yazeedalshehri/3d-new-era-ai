//! The window's fonts: egui's own, the Phosphor icons, and Vazirmatn for
//! Arabic.
//!
//! egui shapes text with `HarfBuzz` (`harfrust`), which joins Arabic letters and
//! lays a run out right to left, but it splits runs by font face. With the
//! Latin font first, every space comes from it, so each Arabic word would be
//! a run of its own and the words would read left to right. In Arabic the
//! Arabic font goes first, so a whole sentence is one run and reads right to
//! left; in the other languages it is only a fallback, so a name typed in
//! Arabic still shows.

use crate::i18n::Lang;

/// Vazirmatn UI, SIL Open Font License 1.1 (see `assets/Vazirmatn-OFL.txt`).
const VAZIRMATN: &[u8] = include_bytes!("../assets/Vazirmatn-UI-Regular.ttf");
const NAME: &str = "vazirmatn";

/// The font set for `lang`.
pub(crate) fn definitions(lang: Lang) -> egui::FontDefinitions {
    let mut fonts = egui::FontDefinitions::default();
    crate::icon::add_to_fonts(&mut fonts);
    fonts
        .font_data
        .insert(NAME.into(), egui::FontData::from_static(VAZIRMATN).into());
    for family in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
        if let Some(keys) = fonts.families.get_mut(&family) {
            if lang == Lang::Ar && family == egui::FontFamily::Proportional {
                keys.insert(0, NAME.into());
            } else {
                keys.push(NAME.into());
            }
        }
    }
    fonts
}

/// Sets the fonts for the current language, when this window's fonts were
/// set for another one (or not yet).
pub(crate) fn sync(ctx: &egui::Context) {
    let lang = crate::i18n::lang();
    let id = egui::Id::unique("newera-fonts-lang");
    if ctx.data(|d| d.get_temp::<Lang>(id)) != Some(lang) {
        ctx.data_mut(|d| d.insert_temp(id, lang));
        ctx.set_fonts(definitions(lang));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arabic_comes_first_only_in_arabic() {
        let first = |lang| definitions(lang).families[&egui::FontFamily::Proportional][0].clone();
        assert_eq!(first(Lang::Ar), NAME);
        for lang in Lang::ALL.into_iter().filter(|l| *l != Lang::Ar) {
            assert_ne!(first(lang), NAME);
            // Still there as a fallback, for names typed in Arabic.
            assert!(
                definitions(lang).families[&egui::FontFamily::Proportional]
                    .iter()
                    .any(|k| k == NAME)
            );
        }
    }
}
