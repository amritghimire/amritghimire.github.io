export class Category {
  private _name: string;
  private _slug: string;
  private _createdAt: Date;
  private _updatedAt: Date;

  constructor(data: { [index: string]: string }) {
    this._name = data.title || "";
    this._slug = data.slug || "";
    this._createdAt = new Date(data.createdAt);
    this._updatedAt = new Date(data.updatedAt);
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

  get slug(): string {
    return this._slug;
  }

  set slug(value: string) {
    this._slug = value;
  }

  get name(): string {
    return this._name;
  }

  set name(value: string) {
    this._name = value;
  }
}
