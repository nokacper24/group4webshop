import { Route, Routes } from "react-router-dom";
import ManageLicenseAccess from "./managing/ManageLicenseAccess";
import MyAccount from "./MyAccount";
import SignIn from "./SignIn";
import CreateCompanyAccount from "./register/CreateCompanyAccount";
import AdminCompanyLicenses from "./admin/AdminCompanyLicenses";
import ManageProducts from "./admin/ManageProducts";
import ManageUsers from "./admin/ManageUsers";

/**
 * The user Profile page.
 * If the user is already signed in, show the My Account page.
 * If the user is not signed in, show the Sign In page.
 *
 * @returns The Profile page component.
 */
export default function Profile() {
  return (
    <>
      <Routes>
        <Route path="/" element={<SignIn />} />
        <Route path="/create-account/*" element={<CreateCompanyAccount />} />
        <Route path="/:userId" element={<MyAccount />} />
        <Route
          path="/manage-license/:licenseId"
          element={<ManageLicenseAccess />}
        />
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
