export const config: { [index: string]: string } = {
  baseUrl: "http://0.0.0.0:8000",
  websiteTitle: ""
};

export const metaConfigurations: {
  [index: string]: { title: string; [index: string]: string };
} = {
  localhost: {
    title: ""
  }
};

export const siteConfigurations: { [index: string]: string } = {
  localhost: "default"
};
