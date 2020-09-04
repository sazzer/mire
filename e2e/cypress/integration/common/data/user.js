import { v4 as uuidv4 } from "uuid";

export class SeedUser {
  constructor(data) {
    this._data = {
      "user id": uuidv4(),
      version: uuidv4(),
      created: new Date(),
      updated: new Date(),
      email: `${uuidv4()}@example.com`,
      "display name": uuidv4(),
      credentials: "",
      ...data,
    };
  }

  async sql() {
    return "INSERT INTO users(user_id, version, created, updated, email, display_name, authentications) VALUES ($1, $2, $3, $4, $5, $6, $7)";
  }

  async binds() {
    const credentials = this._data.credentials
      .split(",")
      .filter((entry) => entry.length > 0)
      .map((entry) => entry.split("/", 3))
      .map(([provider, providerId, displayName]) => {
        return {
          authentication_provider: provider,
          authentication_id: providerId,
          display_name: displayName,
        };
      });
    return [
      this._data["user id"],
      this._data["version"],
      this._data["created"],
      this._data["updated"],
      this._data["email"],
      this._data["display name"],
      JSON.stringify(credentials),
    ];
  }
}
