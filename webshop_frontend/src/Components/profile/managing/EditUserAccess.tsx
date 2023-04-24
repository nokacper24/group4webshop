import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { FullLicenseInfo, User } from "../../../Interfaces";
import {
  fetchLicensesForUser,
  fetchLicensesForUserNoAccess,
  fetchUser,
} from "../../../ApiController";
import ToggleTable, {
  ToggleTableHeaderProps,
  ToggleTableRowProps,
} from "../toggle-table/ToggleTable";

type LicenseAccessProps = {
  license: FullLicenseInfo;
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
  const [newLicensesAccess, setNewLicensesAccess] = useState<
    LicenseAccessProps[]
  >([]);

  const headers: ToggleTableHeaderProps = {
    text: ["License", "Start", "End", "Amount", "Active", ""],
  };
  const [rows, setRows] = useState<ToggleTableRowProps[]>([]);
  const handleClick = (checked: boolean, id: string) => {
    console.log("Checked: ", checked, "ID: ", id);
  };

  const createRowsFromLicenses = () => {
    let tempRows: ToggleTableRowProps[] = [];

    licenses.forEach((license) => {
      let lic = license.license;
      let tempRow: ToggleTableRowProps["row"] = {
        text: [
          lic.display_name,
          new Date(lic.start_date).toLocaleDateString(),
          new Date(lic.end_date).toLocaleDateString(),
          lic.amount.toString(),
          lic.valid.toString(),
        ],
        toggleOn: license.access,
      };
      tempRows.push({ row: tempRow });
    });

    return tempRows;
  };

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

              // Add licenses with access to temporary list and set access to true
              licensesWithAccess.forEach((license) => {
                // TODO: Possible to SetLicenses for every single license instead of pushing to a temp array?
                tempLicenses.push({
                  license: license,
                  access: true,
                });
              });

              // Add licenses without access to temporary list and set access to false
              licensesWithoutAccess.forEach((license) => {
                tempLicenses.push({
                  license: license,
                  access: false,
                });
              });

              setLicenses(tempLicenses);
            }
          );
        });
      }
    });
  }, []);

  useEffect(() => {
    setRows(createRowsFromLicenses());
  }, [licenses]);

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
        <ToggleTable headers={headers} rows={rows} handleClick={handleClick} />
      </section>
    </>
  );
}
