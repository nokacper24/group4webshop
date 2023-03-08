import { useEffect, useState } from "react";
import { SelectTableRowProps } from "../managing/ManageLicenseAccess";
import SelectTable from "../managing/SelectTable";

type UserProps = {
  id: number;
  email: string;
  companyId: number;
};

export default function ManageUsers() {
  let baseUrl = import.meta.env.VITE_URL + ":" + import.meta.env.VITE_PORT;
  // Check if we are in production mode
  if (import.meta.env.PROD) {
    baseUrl = "";
  }

  const [itHeads, setItHeads] = useState<SelectTableRowProps[]>([]);
  const [defaultUser, setDefaultUsers] = useState<SelectTableRowProps[]>([]);

  const [newItHeads] = useState<Set<number>>(new Set());
  const [newDefaultUsers] = useState<Set<number>>(new Set());

  const updateNewUsersOnRemove = (userId: number) => {
    if (newItHeads.has(userId)) {
      newItHeads.delete(userId);
    } else if (!newDefaultUsers.has(userId)) {
      newDefaultUsers.add(userId);
    }
  };
  const updateNewUsersOnAdd = (userId: number) => {
    if (newDefaultUsers.has(userId)) {
      newDefaultUsers.delete(userId);
    } else if (!newItHeads.has(userId)) {
      newItHeads.add(userId);
    }
  };

  const removeItHead = (index: number) => {
    /* Get user who's being moved from "IT heads" to "default users" */
    let user = itHeadsList.rows[index];

    /* Remove user from the "It heads" list */
    let newItHeadsArray = [
      ...itHeadsList.rows.slice(0, index),
      ...itHeadsList.rows.slice(index + 1),
    ];
    setItHeads(newItHeadsArray);

    /* Add user to the "default users" list */
    defaultUsersList.rows.push(user);
    setDefaultUsers(defaultUsersList.rows);

    updateNewUsersOnRemove(parseInt(user.id));
  };

  const removeSelectedItHeads = (indices: number[]) => {
    let sortedIndices = indices.sort((a, b) => a - b);

    for (let i = sortedIndices.length - 1; i >= 0; i--) {
      let index = sortedIndices[i];
      let user = itHeadsList.rows[index];

      itHeadsList.rows = [
        ...itHeadsList.rows.slice(0, index),
        ...itHeadsList.rows.slice(index + 1),
      ];
      defaultUsersList.rows.push(user);

      updateNewUsersOnRemove(parseInt(user.id));
    }

    setItHeads(itHeadsList.rows);
    setDefaultUsers(defaultUsersList.rows);
  };

  const itHeadsList = {
    header: {
      columns: [{ text: "User" }, { text: "Company" }],
    },
    rows: itHeads,
    button: { text: "Remove", action: removeItHead },
    outsideButtons: [
      { text: "Remove all selected", action: removeSelectedItHeads },
    ],
  };

  const addItHead = (index: number) => {
    /* Get user who's being moved from "default users" to "IT heads" */
    let user = defaultUsersList.rows[index];

    /* Remove user from the "default users" list */
    let newDefaultUsersArray = [
      ...defaultUsersList.rows.slice(0, index),
      ...defaultUsersList.rows.slice(index + 1),
    ];
    setDefaultUsers(newDefaultUsersArray);

    /* Add user to the "IT heads" list */
    itHeadsList.rows.push(user);
    setItHeads(itHeadsList.rows);

    updateNewUsersOnAdd(parseInt(user.id));
  };

  const addSelectedItHeads = (indices: number[]) => {
    let sortedIndices = indices.sort((a, b) => a - b);

    for (let i = sortedIndices.length - 1; i >= 0; i--) {
      let index = sortedIndices[i];
      let user = defaultUsersList.rows[index];

      defaultUsersList.rows = [
        ...defaultUsersList.rows.slice(0, index),
        ...defaultUsersList.rows.slice(index + 1),
      ];
      itHeadsList.rows.push(user);

      updateNewUsersOnAdd(parseInt(user.id));
    }
    setDefaultUsers(defaultUsersList.rows);
    setItHeads(itHeadsList.rows);
  };

  const defaultUsersList = {
    header: {
      columns: [{ text: "User" }, { text: "Company" }],
    },
    rows: defaultUser,
    button: { text: "Add", action: addItHead },
    outsideButtons: [{ text: "Add all selected", action: addSelectedItHeads }],
  };

  const fetchCompanyItHead = async () => {
    const response = await fetch(`${baseUrl}/api/users/role/CompanyItHead`);
    const data = await response.json();
    const users: UserProps[] = data.map((user: any) => {
      return {
        id: user.user_id,
        email: user.email,
        companyId: user.company_id,
      };
    });
    return users;
  };

  const fetchCompanyIt = async () => {
    const response = await fetch(`${baseUrl}/api/users/role/CompanyIt`);
    const data = await response.json();
    const users: UserProps[] = data.map((user: any) => {
      return {
        id: user.user_id,
        email: user.email,
        companyId: user.company_id,
      };
    });
    return users;
  };

  const fetchDefaultUser = async () => {
    const response = await fetch(`${baseUrl}/api/users/role/Default`);
    const data = await response.json();
    const users: UserProps[] = data.map((user: any) => {
      return {
        id: user.user_id,
        email: user.email,
        companyId: user.company_id,
      };
    });
    return users;
  };

  const patchAddItHeadsRequest = () => {
    if (newItHeads.size > 0) {
      fetch(`${baseUrl}/api/user_roles`, {
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
          const status = response.status;
          if (status == 200) {
            location.reload();
          } else {
            alert("Something went wrong when saving new IT heads");
          }
        })
        .catch(() => console.error("Failed to update user roles"));
    }
  };

  const patchAddDefaultUsersRequest = () => {
    if (newDefaultUsers.size > 0) {
      fetch(`${baseUrl}/api/user_roles`, {
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
          const status = response.status;
          if (status == 200) {
            location.reload();
          } else {
            alert("Something went wrong when saving new default users");
          }
        })
        .catch(() => console.error("Failed to update user roles"));
    }
  };

  const handleSave = () => {
    patchAddItHeadsRequest();
    patchAddDefaultUsersRequest();
  };

  useEffect(() => {
    fetchCompanyItHead().then((users) => {
      setItHeads(
        users.map((user) => {
          return {
            id: user.id.toString(),
            columns: [
              { text: user.email },
              { text: user.companyId.toString() },
            ],
          };
        })
      );
    });

    fetchCompanyIt().then((companyIt) => {
      fetchDefaultUser().then((defaultUser) => {
        let users: SelectTableRowProps[] = [];

        companyIt.map((user) => {
          users.push({
            id: user.id.toString(),
            columns: [
              { text: user.email },
              { text: user.companyId.toString() },
            ],
          });
        });

        defaultUser.map((user) => {
          users.push({
            id: user.id.toString(),
            columns: [
              { text: user.email },
              { text: user.companyId.toString() },
            ],
          });
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
          header={itHeadsList.header}
          rows={itHeadsList.rows}
          button={itHeadsList.button}
          outsideButtons={itHeadsList.outsideButtons}
        ></SelectTable>
      </section>
      <section className="container left-aligned">
        <h2>Default users</h2>
        <SelectTable
          header={defaultUsersList.header}
          rows={defaultUsersList.rows}
          button={defaultUsersList.button}
          outsideButtons={defaultUsersList.outsideButtons}
        ></SelectTable>
      </section>
      <section className="container left-aligned">
        <button className="default-button small-button" onClick={handleSave}>
          Save changes
        </button>
      </section>
    </>
  );
}
