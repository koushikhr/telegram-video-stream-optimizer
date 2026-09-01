export interface LanguageOption {
  code: string;
  name: string;
  nativeName: string;
}

export const POPULAR_LANGUAGES: LanguageOption[] = [
  { code: "en", name: "English", nativeName: "English" },
  { code: "es", name: "Spanish", nativeName: "Español" },
  { code: "ja", name: "Japanese", nativeName: "日本語" },
  { code: "hi", name: "Hindi", nativeName: "हिन्दी" },
  { code: "fr", name: "French", nativeName: "Français" },
  { code: "de", name: "German", nativeName: "Deutsch" },
  { code: "it", name: "Italian", nativeName: "Italiano" },
  { code: "pt", name: "Portuguese", nativeName: "Português" },
  { code: "ru", name: "Russian", nativeName: "Русский" },
  { code: "zh", name: "Chinese", nativeName: "中文" },
  { code: "ko", name: "Korean", nativeName: "한국어" },
  { code: "ar", name: "Arabic", nativeName: "العربية" },
  { code: "tr", name: "Turkish", nativeName: "Türkçe" },
  { code: "vi", name: "Vietnamese", nativeName: "Tiếng Việt" },
  { code: "id", name: "Indonesian", nativeName: "Bahasa Indonesia" },
  { code: "th", name: "Thai", nativeName: "ไทย" },
  { code: "pl", name: "Polish", nativeName: "Polski" },
  { code: "nl", name: "Dutch", nativeName: "Nederlands" },
  { code: "sv", name: "Swedish", nativeName: "Svenska" },
  { code: "uk", name: "Ukrainian", nativeName: "Українська" },
];

export function normalizeLanguageCode(code: string): string {
  if (!code) return "und";
  const clean = code.trim().toLowerCase();
  if (clean === "en" || clean === "eng" || clean === "english") return "en";
  if (clean === "es" || clean === "spa" || clean === "spanish" || clean === "espanol") return "es";
  if (clean === "ja" || clean === "jpn" || clean === "japanese") return "ja";
  if (clean === "hi" || clean === "hin" || clean === "hindi") return "hi";
  if (clean === "fr" || clean === "fre" || clean === "fra" || clean === "french") return "fr";
  if (clean === "de" || clean === "ger" || clean === "deu" || clean === "german") return "de";
  if (clean === "it" || clean === "ita" || clean === "italian") return "it";
  if (clean === "pt" || clean === "por" || clean === "portuguese") return "pt";
  if (clean === "ru" || clean === "rus" || clean === "russian") return "ru";
  if (clean === "zh" || clean === "chi" || clean === "zho" || clean === "chinese") return "zh";
  if (clean === "ko" || clean === "kor" || clean === "korean") return "ko";
  if (clean === "ar" || clean === "ara" || clean === "arabic") return "ar";
  if (clean === "tr" || clean === "tur" || clean === "turkish") return "tr";
  if (clean === "vi" || clean === "vie" || clean === "vietnamese") return "vi";
  if (clean === "id" || clean === "ind" || clean === "indonesian") return "id";
  if (clean === "th" || clean === "tha" || clean === "thai") return "th";
  if (clean === "pl" || clean === "pol" || clean === "polish") return "pl";
  if (clean === "nl" || clean === "dut" || clean === "nld" || clean === "dutch") return "nl";

  for (const lang of POPULAR_LANGUAGES) {
    if (clean === lang.code || clean.startsWith(lang.code) || clean.startsWith(lang.name.toLowerCase())) {
      return lang.code;
    }
  }
  return clean;
}

export function getLanguageName(code: string): string {
  const norm = normalizeLanguageCode(code);
  const found = POPULAR_LANGUAGES.find((l) => l.code === norm);
  if (found) {
    return `${found.name} (${found.code})`;
  }
  if (norm === "und" || norm === "") {
    return "Undetermined";
  }
  return code.toUpperCase();
}
