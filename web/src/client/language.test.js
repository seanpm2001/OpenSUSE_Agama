/*
 * Copyright (c) [2022] SUSE LLC
 *
 * All Rights Reserved.
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of version 2 of the GNU General Public License as published
 * by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for
 * more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, contact SUSE LLC.
 *
 * To contact SUSE LLC about this file by physical or electronic mail, you may
 * find current contact information at www.suse.com.
 */

// @ts-check

import { LanguageClient } from "./language";
import { DBusClient } from "./dbus";

const LANGUAGE_IFACE = "org.opensuse.DInstaller.Language1";

const dbusClient = new DBusClient("");
const langProxy = {
  wait: jest.fn(),
  AvailableLanguages: [
    ["cs_CZ", "Cestina", {}]
  ]
};

beforeEach(() => {
  dbusClient.proxy = jest.fn().mockImplementation(iface => {
    if (iface === LANGUAGE_IFACE) return langProxy;
  });
});

describe("#getLanguages", () => {
  it("returns the list of available languages", async () => {
    const client = new LanguageClient(dbusClient);
    const availableLanguages = await client.getLanguages();
    expect(availableLanguages).toEqual([{ id: "cs_CZ", name: "Cestina" }]);
  });
});
