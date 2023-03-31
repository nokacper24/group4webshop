import { useState, useEffect } from "react";
import { Link, useParams } from "react-router-dom";
import { User } from "../../Interfaces";
import LicenseList from "./managing/LicenseList";

/**
 * Represents the My Account page.
 * Contains information about the user acccount and owned licenses.
 *
 * @returns The My Account page component.
 */
export default function MyAccount() {
  let baseUrl = import.meta.env.VITE_URL + ":" + import.meta.env.VITE_PORT;
  // Check if we are in production mode
  if (import.meta.env.PROD) {
    baseUrl = "";
  }

  const { userId } = useParams();
  const [user, setUser] = useState<User>();

  const fetchUser = async () => {
    const response = await fetch(`${baseUrl}/api/priv/users/${userId}`);
    const data = await response.json();
    const user: User = data;
    setUser(user);
    return user;
  };

  useEffect(() => {
    fetchUser();
  }, []);

  const companyLicenses = (
    <>
      <h2>Company users</h2>
      <div className="button-container">
        <Link
          to={`../company-users/${user?.company_id}`}
          className="default-button small-button"
        >
          Manage users
        </Link>
      </div>

      <h2>Licenses</h2>
      <LicenseList companyId={user ? user.company_id : -1} />
    </>
  );

  const adminButtons = (
    <div className="button-container">
      <Link
        to="../admin-company-licenses"
        className="default-button small-button"
      >
        Manage company licenses
      </Link>
      <Link to="../admin-users" className="default-button small-button">
        Manage users
      </Link>
      <Link to="../admin-products" className="default-button small-button">
        Manage products
      </Link>
    </div>
  );

  let userRoleSection;
  switch (user?.role) {
    case "Admin":
      userRoleSection = adminButtons;
      break;
    case "CompanyItHead" || "CompanyIt":
      userRoleSection = companyLicenses;
      break;
    default:
      userRoleSection = <p>Placeholder</p>;
  }

  return (
    <>
      <section className="container left-aligned">
        <h1>My account</h1>
        <div className="user-details">
          <p>
            E-mail: {user?.email} <br></br>
          </p>
          <Link className="default-button small-button" to="edit">
            Edit profile
          </Link>
        </div>
      </section>
      <section className="container left-aligned">{userRoleSection}</section>
    </>
  );
}
