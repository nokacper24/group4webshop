import { useState, useEffect } from "react";
import { useParams } from "react-router-dom";
import LicenseList from "./managing/LicenseList";

let baseUrl = import.meta.env.VITE_URL + ":" + import.meta.env.VITE_PORT;

type UserProps = {
  email: string;
  companyId: string;
};

/**
 * Represents the My Account page.
 * Contains information about the user acccount and owned licenses.
 *
 * @returns The My Account page component.
 */
export default function MyAccount() {
  const { id } = useParams();
  const [user, setUser] = useState<UserProps>();

  const fetchUser = async () => {
    const response = await fetch(`${baseUrl}/api/users/${id}`);
    const data = await response.json();
    const user = {
      email: data.email,
      companyId: data.company_id,
    };
    setUser(user);
  };

  useEffect(() => {
    fetchUser();
  }, []);

  return (
    <>
      <section className="container left-aligned">
        <h1>My account</h1>
        <div className="user-details">
          <p>
            E-mail: {user?.email} <br></br>
            Company: {user?.companyId}
            <br></br>
            <a href="">Reset password</a>
          </p>
          <button className="default-button small-button">Edit</button>
        </div>
      </section>
      <section className="container left-aligned">
        <h2>Licenses</h2>
        <LicenseList />
      </section>
    </>
  );
}
