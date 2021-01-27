export interface InnerContents {
  id?: string;
  blog?: string;
  order?: number;
  site?: string;
  created_at?: string;
  updated_at?: string;
  box?: string;
  media?: string;
  link?: string;
  header?: string;
  footer?: string;
  image?: string;
  extra_content?: string;
  background?: string;
  text_color?: string;
}

export interface Contents extends InnerContents {
  body?: string;
  text?: string;
  cardcontent_set?: Array<InnerContents>;
  imagecontent_set?: Array<InnerContents>;
  title?: string;
  count?: number;
  params?: object;
  style?: string;
  items?: string;
  text_before_image?: boolean;
}

export class ContentsType implements Contents {
  id?: string;
  blog?: string;
  order?: number;
  site?: string;
  created_at?: string;
  updated_at?: string;
  box?: string;
  media?: string;
  link?: string;
  header?: string;
  footer?: string;
  image?: string;
  extra_content?: string;
  background?: string;
  text_color?: string;
  body?: string;
  text?: string;
  cardcontent_set?: Array<InnerContents>;
  imagecontent_set?: Array<InnerContents>;
  title?: string;
  count?: number;
  params?: object;
  style?: string;
  items?: string;
  text_before_image?: boolean;
}
