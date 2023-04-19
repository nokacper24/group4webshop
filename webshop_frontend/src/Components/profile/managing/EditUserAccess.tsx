import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { License, User } from "../../../Interfaces";
import {
  fetchLicensesForUser,
  fetchLicensesForUserNoAccess,
  fetchProduct,
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
  const [licenses, setLicenses] = useState<LicenseAccessProps[]>([]);
  const [newLicenseAccess, setNewLicenseAccess] = useState<
    LicenseAccessProps[]
  >([]);

  useEffect(() => {
    // Get user
    fetchUser(userId!).then((user) => {
      setUser(user);
      // If user's ID is found
      if (user.user_id) {
        // Get user's licenses with and without access
        fetchLicensesForUser(user.user_id).then((licensesWithAccess) => {
          fetchLicensesForUserNoAccess(user.user_id).then(
            (licensesWithoutAccess) => {
              let tempLicenses: LicenseAccessProps[] = [];

              licensesWithAccess.forEach((license) => {
                tempLicenses.push({
                  license: license,
                  access: true,
                });
              });

              licensesWithoutAccess.forEach((license) => {
                tempLicenses.push({
                  license: license,
                  access: false,
                });
              });

              // Update all licenses to include license's product name
              tempLicenses.forEach((license) => {
                fetchProduct(license.license.product_id).then((product) => {
                  license.license.product_name = product.display_name;
                });
              });

              setLicenses(tempLicenses);
            }
          );
        });
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
