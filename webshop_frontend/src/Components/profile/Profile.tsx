import React from "react";
import { Route, Routes } from "react-router-dom";
import ManageLicenseAccess from "./managing/ManageLicenseAccess";
import MyAccount from "./MyAccount";
import SignIn from "./SignIn";

/**
 * The user Profile page.
 * If the user is already signed in, show the My Account page.
 * If the user is not signed in, show the Sign In page.
 *
 * @returns The Profile page component.
 */
export default function Profile() {
  return (
    <React.Fragment>
      <Routes>
        <Route path="/" element={<SignIn />} />
        <Route path="/:id" element={<MyAccount />} />
        <Route path="manage-license/:id" element={<ManageLicenseAccess />} />
      </Routes>
    </React.Fragment>
  );
}
