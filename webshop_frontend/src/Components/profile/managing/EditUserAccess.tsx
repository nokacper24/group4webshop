import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { License, LicenseVital, User } from "../../../Interfaces";
import {
  fetchCompanyLicenses,
  fetchLicensesForUser,
  fetchUser,
} from "../../../ApiController";
import ToggleTable from "../toggle-table/ToggleTable";

type LicenseAccessProps = {
  license: License;
  access: boolean;
};

/**
 * Represents a page for editing a user's access to the company's licenses.
 *
 * @returns An Edit User Access component.
 */
export default function EditUserAccess() {
  const { userId } = useParams();
  const [user, setUser] = useState<User>();
  const [companyLicenses, setCompanyLicenses] =
    useState<LicenseAccessProps[]>();
  const [userLicenses, setUserLicenses] = useState<LicenseVital[]>();

  useEffect(() => {
    fetchUser(userId!).then((user) => {
      setUser(user);
      if (user.company_id && user.user_id) {
        fetchCompanyLicenses(user.company_id).then(
          (companyLicenses: License[]) => {
            fetchLicensesForUser(user.user_id).then(
              (userLicenses: LicenseVital[]) => {
                setUserLicenses(userLicenses);

                let licenses: LicenseAccessProps[] = [];
                let licenseIds: number[] = [];

                userLicenses.forEach((license) => {
                  licenseIds.push(license.license_id);
                });

                companyLicenses.forEach((license: License) => {
                  if (licenseIds.includes(license.license_id)) {
                    licenses.push();
                  }
                });
              }
            );
          }
        );
      }
    });
  }, []);

  return (
    <>
      <section className="container left-aligned">
        <h1>User access</h1>
        <p>
          Enable or disable a user's access to their company's licenses.
          Remember that some licenses might already have max total of users.
        </p>
        <p>
          For user: <b>{user?.email}</b>
        </p>
        <ToggleTable />
      </section>
    </>
  );
}
