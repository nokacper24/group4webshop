import { Route, Routes } from "react-router-dom";
import ManageLicenseAccess from "./managing/ManageLicenseAccess";
import MyAccount from "./MyAccount";
import SignIn from "./SignIn";
import Spinner from "../utils/utils";
import AdminCompanyLicenses from "./admin/AdminCompanyLicenses";
import ManageProducts from "./admin/ManageProducts";
import ManageUsers from "./admin/ManageUsers";
import CompanyUsers from "./managing/CompanyUsers";
import { useEffect, useState } from "react";
import EditProfile from "./EditProfile";
import EditUserAccess from "./managing/EditUserAccess";
import PageNotFound from "../PageNotFound";
import { FetchError, fetchMe } from "../../ApiController";
import { MeUser } from "../../Interfaces";
import { ErrorMessage } from "../ErrorMessage";
import ManageProductPage from "./admin/product-managing/ManageProductPage";

/**
 * The user Profile page.
 * If the user is already signed in, show the My Account page.
 * If the user is not signed in, show the Sign In page.
 *
 * @returns The Profile page component.
 */
export default function Profile() {
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);
  const [user, setUser] = useState<MeUser>();

  useEffect(() => {
    fetchMe()
      .then((user: MeUser) => {
        setUser(user);
        setLoading(false);
      })
      .catch((error: FetchError) => {
        if (error.status === 401) {
          setLoading(false);
        } else {
          setError(error);
          setLoading(false);
        }
      });
  }, []);

  return (
    <>
      {loading && <Spinner />}
      {error && <ErrorMessage message={error.message} />}
      {!loading && !error && (
        <Routes>
          <Route
            path="/"
            element={user ? <MyAccount user={user} /> : <SignIn />}
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
          {(user?.role == "CompanyItHead" || user?.role == "CompanyIt") && (
            <>
              <Route
                path="/manage-license/:licenseId"
                element={<ManageLicenseAccess />}
              />
              <Route
                path="/company-users/:companyId"
                element={<CompanyUsers />}
              />
              <Route
                path="/:userId/license-access"
                element={<EditUserAccess />}
              />
            </>
          )}

          {/* Admin */}
          {user?.role == "Admin" && (
            <>
              <Route
                path="/admin-company-licenses"
                element={<AdminCompanyLicenses />}
              ></Route>
              <Route
                path="/admin-products"
                element={<ManageProducts />}
              ></Route>
              <Route path="/admin-users" element={<ManageUsers />}></Route>
              <Route
                path="/product/manage/:productId"
                element={<ManageProductPage />}
              />
              <Route path="/product/create" element={<ManageProductPage />} />
            </>
          )}

          <Route path="*" element={<PageNotFound />} />
        </Routes>
      )}
    </>
  );
}
