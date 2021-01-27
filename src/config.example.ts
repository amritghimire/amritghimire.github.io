export const config: { [index: string]: string } = {
  baseUrl: "${BASE_URL}",
  websiteTitle: "${WEBSITE_TITLE}"
};

export const metaConfigurations: {
  [index: string]: { title: string; [index: string]: string };
} = {
  "${WEBSITE_DOMAIN}": {
    title: "${WEBSITE_TITLE}",
    description: "${WEBSITE_META_DESCRIPTION}"
  }
};

export const siteConfigurations: { [index: string]: string } = {
  "${WEBSITE_DOMAIN}": "${SITE_CONFIG}"
};
