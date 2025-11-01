// i18n configuration for SynapseNet Mobile
import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';
import en from './locales/en.json';
import ru from './locales/ru.json';
import es from './locales/es.json';
import zh from './locales/zh.json';

const resources = {
  en: { translation: en },
  ru: { translation: ru },
  es: { translation: es },
  zh: { translation: zh },
};

i18n
  .use(initReactI18next)
  .init({
    resources,
    lng: 'en', // default language
    fallbackLng: 'en',
    interpolation: {
      escapeValue: false,
    },
  });

export default i18n;
