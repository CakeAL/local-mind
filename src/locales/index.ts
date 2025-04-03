export async function loadLocalemessages(locale: string) {
  try {
    const messages = await import(`./${locale}.json`);
    return messages.default;
  } catch (error) {
    console.error(`Failed to load ${locale} locale`, error);
    return {};
  }
}
