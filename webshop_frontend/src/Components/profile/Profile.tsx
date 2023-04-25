import { Route, Routes } from "react-router-dom";
import ManageLicenseAccess from "./managing/ManageLicenseAccess";
import MyAccount from "./MyAccount";
import SignIn from "./SignIn";
import AdminCompanyLicenses from "./admin/AdminCompanyLicenses";
import ManageProducts from "./admin/ManageProducts";
import ManageUsers from "./admin/ManageUsers";
import CompanyUsers from "./managing/CompanyUsers";
import { useEffect, useState } from "react";
import EditProfile from "./EditProfile";
import EditUserAccess from "./managing/EditUserAccess";
import PageNotFound from "../PageNotFound";
import { checkSignInStatus, fetchMe } from "../../ApiController";
import { MeUser } from "../../Interfaces";

/**
 * The user Profile page.
 * If the user is already signed in, show the My Account page.
 * If the user is not signed in, show the Sign In page.
 *
 * @returns The Profile page component.
 */
export default function Profile() {
  const [signedIn, setSignedIn] = useState<boolean>(false);
  const [user, setUser] = useState<MeUser>();

  useEffect(() => {
    checkSignInStatus().then((signedIn: boolean) => {
      setSignedIn(signedIn);
      if (signedIn) {
        fetchMe().then((user: MeUser) => setUser(user));
      }
    });
  }, [signedIn]);

  return (
    <>
      <Routes>
        <Route
          path="/"
          element={signedIn && user ? <MyAccount user={user} /> : <SignIn />}
        />

        <Route
          path="/edit"
          element={
            user ? (
              <EditProfile user={user} />
            ) : (
              <section className="container">Error</section>
            )
          }
        />

        {/* License manager */}
        <Route
          path="/manage-license/:licenseId"
          element={<ManageLicenseAccess />}
        />
        <Route path="/company-users/:companyId" element={<CompanyUsers />} />
        <Route path="/:userId/license-access" element={<EditUserAccess />} />

        {/* Admin */}
        <Route
          path="/admin-company-licenses"
          element={<AdminCompanyLicenses />}
        ></Route>
        <Route path="/admin-products" element={<ManageProducts />}></Route>
        <Route path="/admin-users" element={<ManageUsers />}></Route>

        <Route path="*" element={<PageNotFound />} />
      </Routes>
    </>
  );
}
