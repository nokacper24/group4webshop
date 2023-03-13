import { useState, useEffect } from "react";
import { Link, useParams } from "react-router-dom";
import LicenseList from "./managing/LicenseList";

export type UserProps = {
  userId: string;
  email: string;
  companyId: string;
  role: string;
};

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
  const [user, setUser] = useState<UserProps>({
    userId: "",
    email: "",
    companyId: "",
    role: "Default",
  });

  const fetchUser = async () => {
    const response = await fetch(`${baseUrl}/api/users/${userId}`);
    const data = await response.json();
    const user = {
      userId: data.user_id,
      email: data.email,
      companyId: data.company_id,
      role: data.role,
    };
    setUser(user);
    return user;
  };

  useEffect(() => {
    fetchUser();
  }, []);

  const companyLicenses = (
    <>
      <h2>Licenses</h2>
      <LicenseList companyId={user.companyId} />
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
          <button className="default-button small-button">Edit profile</button>
        </div>
      </section>
      <section className="container left-aligned">{userRoleSection}</section>
    </>
  );
}
