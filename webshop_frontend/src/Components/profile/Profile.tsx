import { Navigate, Route, Routes } from "react-router-dom";
import ManageLicenseAccess from "./managing/ManageLicenseAccess";
import MyAccount from "./MyAccount";
import SignIn from "./SignIn";
import CreateCompanyAccount from "./register/CreateCompanyAccount";
import AdminCompanyLicenses from "./admin/AdminCompanyLicenses";
import ManageProducts from "./admin/ManageProducts";
import ManageUsers from "./admin/ManageUsers";
import CompanyUsers from "./managing/CompanyUsers";
import { useEffect, useState } from "react";
import { User } from "../../Interfaces";

let baseUrl = import.meta.env.VITE_URL + ":" + import.meta.env.VITE_PORT + "/";
// check if we are in production mode
if (import.meta.env.PROD) {
  baseUrl = "../";
}

/**
 * The user Profile page.
 * If the user is already signed in, show the My Account page.
 * If the user is not signed in, show the Sign In page.
 *
 * @returns The Profile page component.
 */
export default function Profile() {
  const [user, setUser] = useState<User>();

  // check sign in status
  const checkSignInStatus = async () => {
    let result = await fetch(baseUrl + "api/priv/logged_in", {
      method: "GET",
      credentials: "include",
    });

    if (result.status === 200) {
      const data: User = await result.json();
      setUser(data);

      return true;
    } else {
      return false;
    }
  };

  const [signedIn, setSignedIn] = useState<boolean>(false);

  useEffect(() => {
    checkSignInStatus().then((result) => {
      setSignedIn(result);
    });
  }, [signedIn]);

  return (
    <>
      <Routes>
        <Route
          path="/"
          element={signedIn ? <Navigate to={`${user?.user_id}`} /> : <SignIn />}
        />
        <Route path="/create-account/*" element={<CreateCompanyAccount />} />
        <Route path="/:userId" element={<MyAccount />} />

        {/* License manager */}
        <Route
          path="/manage-license/:licenseId"
          element={<ManageLicenseAccess />}
        />
        <Route path="/company-users/:companyId" element={<CompanyUsers />} />

        {/* Admin */}
        <Route
          path="/admin-company-licenses"
          element={<AdminCompanyLicenses />}
        ></Route>
        <Route path="/admin-products" element={<ManageProducts />}></Route>
        <Route path="/admin-users" element={<ManageUsers />}></Route>
      </Routes>
    </>
  );
}
