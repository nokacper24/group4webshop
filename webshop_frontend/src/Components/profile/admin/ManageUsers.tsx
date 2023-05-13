import { useEffect, useState } from "react";
import { User } from "../../../Interfaces";
import SelectTable, {
  SelectTableProps,
  SelectTableRowProps,
} from "../select-table/SelectTable";
import {
  createSelectTableProps,
  createRowProps,
  updateNewChanges,
  moveItemBetweenTables,
  moveItemsBetweenTables,
} from "../select-table/SelectTableFunctions";
import { useNavigate } from "react-router-dom";
import {
  fetchCompanyIt,
  fetchCompanyItHead,
  fetchDefaultUser,
} from "../../../ApiController";

/**
 * A Manage Users page.
 * The website admins can choose which users get to be IT heads for their companies.
 *
 * @returns A Manage Users page component.
 */
export default function ManageUsers() {
  let baseUrl = import.meta.env.VITE_URL + ":" + import.meta.env.VITE_PORT;
  // Check if we are in production mode
  if (import.meta.env.PROD) {
    baseUrl = "";
  }

  const navigate = useNavigate();

  const [itHeads, setItHeads] = useState<SelectTableRowProps[]>([]);
  const [newItHeads] = useState<Set<string>>(new Set());

  const [defaultUsers, setDefaultUsers] = useState<SelectTableRowProps[]>([]);
  const [newDefaultUsers] = useState<Set<string>>(new Set());

  /**
   * Update the lists of new IT heads and default users
   * when a user gets removed from the list of IT heads.
   *
   * @param user The user to update.
   */
  const updateNewUsersOnRemove = (user: SelectTableRowProps) => {
    updateNewChanges(user, newItHeads, newDefaultUsers);
  };

  /**
   * Update the lists of new IT heads and default users
   * when a user gets added to the list of IT heads.
   *
   * @param user The user to update.
   */
  const updateNewUsersOnAdd = (user: SelectTableRowProps) => {
    updateNewChanges(user, newDefaultUsers, newItHeads);
  };

  /**
   * Remove a user from the list of IT heads.
   *
   * @param index The index of the user in the list of IT  heads.
   */
  const removeItHead = (id: string) => {
    let user = moveItemBetweenTables(
      id,
      itHeadsTable,
      defaultUsersTable,
      setItHeads,
      setDefaultUsers
    );

    updateNewUsersOnRemove(user);
  };

  /**
   * Remove all the selected users from the list of IT heads.
   *
   * @param indices The indices of the users in the list of IT  heads.
   */
  const removeSelectedItHeads = (ids: string[]) => {
    moveItemsBetweenTables(
      ids,
      itHeadsTable,
      defaultUsersTable,
      setItHeads,
      setDefaultUsers,
      newItHeads,
      newDefaultUsers
    );
  };

  const itHeadsTable: SelectTableProps = createSelectTableProps(
    ["User", "Company"],
    itHeads,
    "Remove",
    removeItHead,
    new Map([["Remove all selected", removeSelectedItHeads]])
  );

  /**
   * Add a user to the list of IT heads.
   *
   * @param index The index of the user in the list of IT  heads.
   */
  const addItHead = (id: string) => {
    let user = moveItemBetweenTables(
      id,
      defaultUsersTable,
      itHeadsTable,
      setDefaultUsers,
      setItHeads
    );

    updateNewUsersOnAdd(user);
  };

  /**
   * Add all the selected users to the list of IT heads.
   *
   * @param ids The indices of the users in the list of IT  heads.
   */
  const addSelectedItHeads = (ids: string[]) => {
    moveItemsBetweenTables(
      ids,
      defaultUsersTable,
      itHeadsTable,
      setDefaultUsers,
      setItHeads,
      newDefaultUsers,
      newItHeads
    );
  };

  const defaultUsersTable: SelectTableProps = createSelectTableProps(
    ["Users", "Company"],
    defaultUsers,
    "Add",
    addItHead,
    new Map([["Add all selected", addSelectedItHeads]])
  );

  /**
   * Send a PATCH request to give some users the 'Company IT Head' role.
   */
  const sendPatchAddItHeadsRequest = () => {
    if (newItHeads.size > 0) {
      fetch(`${baseUrl}/api/priv/user_roles`, {
        method: "PATCH",
        headers: {
          Accept: "application/json",
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          users: Array.from(newItHeads, (id) => {
            return {
              user_id: id,
              role: "CompanyItHead",
            };
          }),
        }),
      })
        .then((response) => {
          if (response.ok) {
            // Refresh
            navigate(0);
          } else {
            alert("Something went wrong when saving new IT heads");
          }
        })
        .catch(() => alert("Failed to save users"));
    }
  };

  /**
   * Send a PATCH request to give some users the 'Default' role.
   */
  const patchAddDefaultUsersRequest = () => {
    if (newDefaultUsers.size > 0) {
      fetch(`${baseUrl}/api/priv/user_roles`, {
        method: "PATCH",
        headers: {
          Accept: "application/json",
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          users: Array.from(newDefaultUsers, (id) => {
            return {
              user_id: id,
              role: "Default",
            };
          }),
        }),
      })
        .then((response) => {
          if (response.ok) {
            // Refresh
            navigate(0);
          } else {
            alert("Something went wrong when saving new default users");
          }
        })
        .catch(() => alert("Failed to save users"));
    }
  };

  /**
   * Save the changes made to the user lists.
   */
  const handleSave = () => {
    sendPatchAddItHeadsRequest();
    patchAddDefaultUsersRequest();
  };

  useEffect(() => {
    fetchCompanyItHead().then((users) => {
      setItHeads(
        users.map((user: User) => {
          return createRowProps(user.user_id, [
            user.email,
            user.company_id.toString(),
          ]);
        })
      );
    });

    fetchCompanyIt().then((companyIt) => {
      fetchDefaultUser().then((defaultUser) => {
        let users: SelectTableRowProps[] = [];

        companyIt.map((user: User) => {
          users.push(
            createRowProps(user.user_id, [
              user.email,
              user.company_id.toString(),
            ])
          );
        });

        defaultUser.map((user: User) => {
          users.push(
            createRowProps(user.user_id, [
              user.email,
              user.company_id.toString(),
            ])
          );
        });

        setDefaultUsers(users);
      });
    });
  }, []);

  return (
    <>
      <section className="container left-aligned">
        <h1>Manage users</h1>
        <p>
          Give users the 'Company IT Head' role to give them the most access for
          managing their company. Both 'Company IT Head' and 'Company IT' are
          license managers, and can give users within their company access to
          the company's licenses. They can also overview all of the company's
          users and licenses.
        </p>
      </section>
      <section className="container left-aligned">
        <h2>Company IT heads</h2>
        <SelectTable
          header={itHeadsTable.header}
          rows={itHeadsTable.rows}
          button={itHeadsTable.button}
          outsideButtons={itHeadsTable.outsideButtons}
        />
      </section>
      <section className="container left-aligned">
        <h2>Default users</h2>
        <SelectTable
          header={defaultUsersTable.header}
          rows={defaultUsersTable.rows}
          button={defaultUsersTable.button}
          outsideButtons={defaultUsersTable.outsideButtons}
        />
      </section>
      <section className="container left-aligned">
        <button className="default-button small-button" onClick={handleSave}>
          Save changes
        </button>
      </section>
    </>
  );
}
