import { Category } from "@/models/category";
import { Contents } from "@/models/contents";

export interface BlogData {
  id: string;
  title: string;
  slug: string;
  excerpt: string;
  status: string;
  createdAt: string;
  updatedAt: string;
  media: string;
  category: { [index: string]: string };
  contents?: Array<Contents>;
}

export class Blog {
  private _id: string;
  private _title: string;
  private _slug: string;
  private _excerpt: string;
  private _status: string;
  private _createdAt: Date;
  private _updatedAt: Date;
  private _category: Category;
  private _contents?: Array<Contents>;
  private _media?: string;

  constructor(data: BlogData) {
    this._id = data.id || "";
    this._title = data.title || "";
    this._slug = data.slug || "";
    this._excerpt = data.excerpt || "";
    this._status = data.status || "";
    this._createdAt = new Date(data.createdAt);
    this._updatedAt = new Date(data.updatedAt);
    this._category = new Category(data.category);
    this._contents = data.contents;
    this._media = data.media || "";
  }

  get id(): string {
    return this._id;
  }

  set id(value: string) {
    this._id = value;
  }

  get updatedAt(): Date {
    return this._updatedAt;
  }

  set updatedAt(value: Date) {
    this._updatedAt = new Date(value);
  }

  get createdAt(): Date {
    return this._createdAt;
  }

  set createdAt(value: Date) {
    this._createdAt = new Date(value);
  }

  get status(): string {
    return this._status;
  }

  set status(value: string) {
    this._status = value;
  }

  get excerpt(): string {
    return this._excerpt;
  }

  set excerpt(value: string) {
    this._excerpt = value;
  }

  get slug(): string {
    return this._slug;
  }

  set slug(value: string) {
    this._slug = value;
  }

  get title(): string {
    return this._title;
  }

  set title(value: string) {
    this._title = value;
  }

  get category(): Category {
    return this._category;
  }

  set category(value: Category) {
    this._category = value;
  }

  get contents(): Array<Contents> {
    return this._contents || [];
  }

  set contents(value: Array<Contents>) {
    this._contents = value;
  }

  get media(): string {
    return this._media || "";
  }

  set media(value: string) {
    this._media = value;
  }
}
