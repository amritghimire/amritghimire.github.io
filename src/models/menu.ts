import { Blog, BlogData } from "@/models/blog";

export interface MenuData {
  id: string;
  name: string;
  order: string;
  blog: BlogData;
  children: Array<object>;
}

export class Menu {
  id: string;
  name: string;
  order: string;
  blog: Blog;
  children: Array<Menu>;

  constructor(data: MenuData) {
    this.id = data.id || "";
    this.name = data.name || "";
    this.order = data.order || "";
    this.blog = new Blog(data.blog as BlogData);
    this.children = (data.children || []) as Array<Menu>;
  }
}
