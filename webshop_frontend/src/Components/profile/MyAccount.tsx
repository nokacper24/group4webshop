import { useState, useEffect } from "react";
import { useParams } from "react-router-dom";
import LicenseList from "./managing/LicenseList";

export type UserProps = {
  email: string;
  companyId: string;
};

export type LicenseProps = {
  licenseId: number;
  valid: boolean;
  startDate: Date;
  endDate: Date;
  amount: number;
  companyId?: number;
  productId: string;
};

/**
 * Represents the My Account page.
 * Contains information about the user acccount and owned licenses.
 *
 * @returns The My Account page component.
 */
export default function MyAccount() {
  const baseUrl = import.meta.env.VITE_URL + ":" + import.meta.env.VITE_PORT;

  const { userId } = useParams();
  const [user, setUser] = useState<UserProps>();
  const [licenses, setLicenses] = useState<LicenseProps[]>([]);

  const fetchUser = async () => {
    const response = await fetch(`${baseUrl}/api/users/${userId}`);
    const data = await response.json();
    const user = {
      email: data.email,
      companyId: data.company_id,
    };
    setUser(user);
    return user;
  };

  const fetchLicenses = async (companyId: string) => {
    const response = await fetch(
      `${baseUrl}/api/company/${companyId}/licenses`
    );
    const data = await response.json();
    const licenses = data.map((license: any) => {
      return {
        licenseId: license.license_id,
        valid: license.valid,
        startDate: license.start_date,
        endDate: license.end_date,
        amount: license.amount,
        productId: license.product_id,
      };
    });
    setLicenses(licenses);
  };

  useEffect(() => {
    fetchUser()
      .then((user) => fetchLicenses(user.companyId))
      .catch((e) =>
        console.log("An error occurred while trying to get the licenses.", e)
      );
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
        <LicenseList licenses={licenses} />
      </section>
    </>
  );
}
