/*
 * Copyright (c) [2024] SUSE LLC
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

import React from "react";
import { _ } from "~/i18n";

import StoragePage from "./StoragePage";
import BootSelection from "./BootSelection";
import DASDPage from "./DASDPage";
import DeviceSelection from "./DeviceSelection";
import ISCSIPage from "./ISCSIPage";
import ProposalPage from "./ProposalPage";
import ZFCPPage from "./ZFCPPage";

// FIXME: Choose a better name
const navigation = [
  // FIXME: use index: true
  { path: "/storage", element: <ProposalPage />, handle: { name: _("Proposal") } },
  { path: "iscsi", element: <ISCSIPage />, handle: { name: _("iSCSI") } }
];

// if (something) {
//   navigation.push({ path: "dasd", element: <DASDPage />, handle: { ... } })
// }
//
// if (somethingElse) {
//   navigation.push({ path: "zfcp", element: <ZFCPPage />, handle: { ... } })
// }

const selectors = [
  { path: "target-device", element: <DeviceSelection /> },
  { path: "booting-partition", element: <BootSelection /> }
];

const routes = {
  path: "/storage",
  element: <StoragePage />,
  handle: {
    name: _("Storage"),
    icon: "hard_drive"
  },
  children: [
    ...navigation,
    ...selectors,
  ]
};

export default routes;
export { navigation, selectors };
