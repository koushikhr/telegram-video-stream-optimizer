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

export function getLanguageName(code: string): string {
  const norm = code.toLowerCase().trim();
  const found = POPULAR_LANGUAGES.find(
    (l) => l.code === norm || norm.startsWith(l.code)
  );
  if (found) {
    return `${found.name} (${found.code})`;
  }
  if (norm === "und" || norm === "") {
    return "Undetermined";
  }
  return code.toUpperCase();
}
