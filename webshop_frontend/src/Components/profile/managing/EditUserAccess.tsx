import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
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

let baseUrl = import.meta.env.VITE_URL + ":" + import.meta.env.VITE_PORT;
// Check if we are in production mode
if (import.meta.env.PROD) {
  baseUrl = "";
}

/**
 * Represents a page for editing a user's access to the company's licenses.
 *
 * @returns An Edit User Access component.
 */
export default function EditUserAccess() {
  const { userId } = useParams();
  const [user, setUser] = useState<User>();
  const [licenses, setLicenses] = useState<Map<string, LicenseAccessProps>>(
    new Map()
  );
  const [newLicensesAccess, setNewLicensesAccess] = useState<
    Map<string, LicenseAccessProps>
  >(new Map());

  const headers: ToggleTableHeaderProps = {
    text: ["License", "Start", "End", "Amount", "Access"],
  };
  const [rows, setRows] = useState<ToggleTableRowProps[]>([]);

  const navigate = useNavigate();

  /**
   * Updates the "user access" status of a license.
   *
   * @param checked Whether the user has access to the license or not.
   * @param id The ID of the license.
   */
  const handleClick = (checked: boolean, id: string) => {
    // Toggle the license access
    let tempLicenses = new Map(licenses);
    tempLicenses.set(id, {
      ...licenses.get(id)!,
      access: checked,
    });
    setLicenses(tempLicenses);

    // Add it to map of all changed license access
    let tempNewAccess = new Map(newLicensesAccess);
    tempNewAccess.set(id, tempLicenses.get(id)!);
    setNewLicensesAccess(tempNewAccess);
  };

  /**
   * Create rows for Toggle Table from the licenses.
   *
   * @returns Rows for the Toggle table.
   */
  const createRowsFromLicenses = () => {
    let tempRows: ToggleTableRowProps[] = [];

    licenses.forEach((license) => {
      let lic = license.license;
      let tempRow: ToggleTableRowProps["row"] = {
        id: lic.license_id.toString(),
        text: [
          lic.display_name,
          new Date(lic.start_date).toLocaleDateString(),
          new Date(lic.end_date).toLocaleDateString(),
          lic.amount.toString(),
        ],
        toggleOn: license.access,
      };
      tempRows.push({ row: tempRow });
    });

    return tempRows;
  };

  /**
   * Get a list from new license map based on if the user should have access or not.
   * Every element in the list has the user ID and license ID.
   *
   * @param access If the user will have access to the license.
   * @returns A list of user licenses.
   */
  const getLicenseListFromMap = (access: boolean) => {
    let tempLicenses: { user_id: number; license_id: number }[] = [];

    newLicensesAccess.forEach((license) => {
      if (license.access == access) {
        tempLicenses.push({
          user_id: parseInt(userId!),
          license_id: license.license.license_id,
        });
      }
    });

    return tempLicenses;
  };

  /**
   * Handle saving user access for licenses.
   */
  const handleSaveAccess = () => {
    addLicensesAccess();
    removeLicensesAccess();
  };

  /**
   * Send a POST request to add user access to licenses.
   */
  const addLicensesAccess = async () => {
    if (newLicensesAccess.size > 0) {
      fetch(`${baseUrl}/api/priv/license_users`, {
        method: "POST",
        headers: {
          Accept: "application/json",
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          users: getLicenseListFromMap(true),
        }),
      })
        .then((response) => {
          handleUpdateLicenseAccess(response);
        })
        .catch(() => alert("Failed to update access."));
    }
  };

  /**
   * Send a DELETE request to remove user access from licenses.
   */
  const removeLicensesAccess = async () => {
    if (newLicensesAccess.size > 0) {
      fetch(`${baseUrl}/api/priv/license_users`, {
        method: "DELETE",
        headers: {
          Accept: "application/json",
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          users: getLicenseListFromMap(false),
        }),
      })
        .then((response) => {
          handleUpdateLicenseAccess(response);
        })
        .catch(() => alert("Failed to update access."));
    }
  };

  /**
   * Handle the response from fetch request when updating license access.
   *
   * @param response
   */
  const handleUpdateLicenseAccess = (response: Response) => {
    let status = response.status;

    if (status == 200 || status == 201) {
      navigate(0);
    } else {
      alert("Something went wrong when updating license access.");
    }
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
              let tempLicenses: Map<string, LicenseAccessProps> = new Map();

              // Add licenses with access to temporary list and set access to true
              licensesWithAccess.forEach((license) => {
                // TODO: Possible to SetLicenses for every single license instead of pushing to a temp array?
                tempLicenses.set(license.license_id.toString(), {
                  license: license,
                  access: true,
                });
              });

              // Add licenses without access to temporary list and set access to false
              licensesWithoutAccess.forEach((license) => {
                tempLicenses.set(license.license_id.toString(), {
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
        <button
          className="default-button small-button"
          onClick={handleSaveAccess}
        >
          Save
        </button>
      </section>
    </>
  );
}
