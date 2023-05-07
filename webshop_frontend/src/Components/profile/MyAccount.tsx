import { Link, useNavigate } from "react-router-dom";
import { MeUser } from "../../Interfaces";
import LicenseList from "./managing/LicenseList";
import { logout } from "../../ApiController";
import UserLicenses from "./UserLicenses";

interface Props {
  user: MeUser;
}

/**
 * Represents the My Account page.
 * Contains information about the user acccount and owned licenses.
 *
 * @returns The My Account page component.
 */
export default function MyAccount(props: Props) {
  const navigate = useNavigate();

  const userLicenses = (
    <>
      <h2>My Licenses</h2>
      <UserLicenses userId={props.user.user_id} />
    </>
  );

  const companyInfo = (
    <>
      <h2>Company users</h2>
      <div className="button-container">
        <Link
          to={`../company-users/${props.user.company_id}`}
          className="default-button small-button"
        >
          Manage users
        </Link>
      </div>

      <h2>Licenses</h2>
      <LicenseList companyId={props.user ? props.user.company_id : NaN} />
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
  switch (props.user?.role) {
    case "Admin":
      userRoleSection = adminButtons;
      break;
    case "CompanyItHead" || "CompanyIt":
      userRoleSection = companyInfo;
      break;
    default:
      userRoleSection = userLicenses;
  }

  return (
    <>
      <section className="container left-aligned">
        <h1>My account</h1>
        <div className="user-details">
          <p>
            E-mail: {props.user?.email} <br />
          </p>
          <Link className="default-button small-button" to="edit">
            Edit profile
          </Link>
        </div>
      </section>
      <section className="container left-aligned">{userRoleSection}</section>
      <section className="container left-aligned">
        <button
          className="default-button small-button"
          onClick={async () => {
            let result = await logout();
            if (result && result.ok) {
              navigate(0);
            } else {
              alert("Could not log out.");
            }
          }}
        >
          Log out
        </button>
      </section>
    </>
  );
}
