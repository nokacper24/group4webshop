import { useEffect, useRef, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import SelectTable, {
  SelectTableProps,
  SelectTableRowProps,
} from "./SelectTable";
import { User } from "../../../Interfaces";
import { createSelectTableProps, createRowProps } from "./SelectTableFunctions";
import { fetchCompanyUsers } from "../../../ApiController";

/**
 * A page for managing company users.
 * Users can be removed from the company, which revokes all their license accesses.
 *
 * @returns A manage company users page.
 */
export default function CompanyUsers() {
  let baseUrl = import.meta.env.VITE_URL + ":" + import.meta.env.VITE_PORT;
  // Check if we are in production mode
  if (import.meta.env.PROD) {
    baseUrl = "";
  }

  const navigate = useNavigate();

  const { companyId } = useParams();

  const [users, setUsers] = useState<SelectTableRowProps[]>([]);
  const [newRemovedUsers] = useState<Set<string>>(new Set());

  const singleEmail = useRef<HTMLInputElement>(null);
  const csvEmail = useRef<HTMLInputElement>(null);

  const editUser = (index: number) => {
    let userId = users[index].id;
    navigate(`/profile/${userId}/license-access`);
  };

  /**
   * Add user as pending to be removed.
   *
   * @param user The user to remove.
   */
  const updateNewRemovedUsers = (userId: string) => {
    newRemovedUsers.add(userId);
  };

  /**
   * Remove selected users from list of users.
   *
   * @param indices The indices of the users in the list.
   */
  const removeUsers = (indices: number[]) => {
    let sortedIndices = indices.sort((a, b) => a - b);

    for (let i = sortedIndices.length - 1; i >= 0; i--) {
      let index = sortedIndices[i];
      let user = usersTable.rows[index];
      usersTable.rows = [
        ...usersTable.rows.slice(0, index),
        ...usersTable.rows.slice(index + 1),
      ];

      updateNewRemovedUsers(user.id);
    }

    setUsers(usersTable.rows);
  };

  const usersTable: SelectTableProps = createSelectTableProps(
    ["User"],
    users,
    "Edit",
    editUser,
    new Map([["Remove selected", removeUsers]])
  );

  /**
   * Send a DELETE request to remove users.
   */
  const sendDeleteUsersRequest = async () => {
    fetch(`${baseUrl}/api/priv/users`, {
      method: "DELETE",
      headers: {
        Accept: "application/json",
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        users: Array.from(newRemovedUsers, (item) => {
          return {
            user_id: item,
          };
        }),
      }),
    })
      .then((response) => {
        const status = response.status;
        if (status == 200) {
          // Refresh
          navigate(0);
        } else {
          alert("Something went wrong when saving users");
        }
      })
      .catch(() => alert("Failed to save users"));
  };

  /**
   * Save the changes made to users.
   */
  const handleSave = () => {
    if (newRemovedUsers.size > 0) {
      if (
        confirm(
          "Are you sure you want to remove the users? This action cannot be reversed."
        )
      ) {
        sendDeleteUsersRequest();
      }
    }
  };

  /**
   * Set the single email input to be empty.
   */
  const resetSingleEmailInput = () => {
    if (singleEmail.current) {
      singleEmail.current.value = "";
    }
  };

  /**
   * Set the CSV email input to be empty.
   */
  const resetCsvEmailInput = () => {
    if (csvEmail.current) {
      csvEmail.current.value = "";
    }
  };

  /**
   * Send a POST request for registering user.
   *
   * @param email The e-mail for the user to be created.
   */
  const sendPostRegisterUserRequest = async (email: string) => {
    const formData = new FormData();
    formData.append("email", email);

    fetch(`${baseUrl}/api/priv/generate_invite`, {
      method: "POST",
      headers: {
        Accept: "application/json",
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        email: email,
        company_id: parseInt(companyId!),
      }),
    })
      .then((response) => {
        const status = response.status;
        if (status == 200) {
          resetSingleEmailInput();
        } else {
          alert("Something went wrong when inviting user");
        }
      })
      .catch(() => alert("Failed to invite user"));
  };

  /**
   * Send a POST request for registering users.
   *
   * @param filePath The file path for the CSV file of users to be created.
   */
  const sendPostRegisterUsersRequest = async () => {
    const formData = new FormData();
    if (csvEmail.current?.files) {
      formData.append("csv", csvEmail.current.files[0]);
    }

    fetch(`${baseUrl}/api/priv/generate_invites`, {
      method: "POST",
      headers: {
        Accept: "multipart/form-data",
      },
      body: formData,
    })
      .then((response) => {
        const status = response.status;
        if (status == 200) {
          resetCsvEmailInput();
          alert("Users invited");
        } else {
          alert("Something went wrong when inviting users");
        }
      })
      .catch(() => alert("Failed to invite users"));
  };

  const handleSubmitSingleEmail = (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    if (singleEmail.current && singleEmail.current.value != "") {
      sendPostRegisterUserRequest(singleEmail.current.value);
    }
  };

  const handleSubmitCsvEmail = (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    if (csvEmail.current && csvEmail.current?.value != "") {
      sendPostRegisterUsersRequest();
    }
  };

  useEffect(() => {
    fetchCompanyUsers(companyId!).then((users: User[]) => {
      let filteredUsers: User[] = [];

      users.map((user: User) => {
        if (user.role != "Admin" && user.role != "CompanyItHead") {
          filteredUsers.push(user);
        }
      });

      setUsers(
        filteredUsers.map((user: User) => {
          return createRowProps(user.user_id, [user.email]);
        })
      );
    });
  }, []);

  return (
    <>
      <section className="container left-aligned">
        <h1>Manage users</h1>
        <p>
          Remove or edit default users. You cannot remove users that have the
          role 'Company IT head' or 'Company IT'. In order to remove 'Company
          IT' users, you first have to remove the role from them.
        </p>
        <p>
          Once saved, these changes cannot be undone. The removed user accounts
          will be deleted. They can create a new account with the same e-mail
          address, but will have to go through all the initial steps again.
        </p>
        <SelectTable
          header={usersTable.header}
          rows={usersTable.rows}
          button={usersTable.button}
          outsideButtons={usersTable.outsideButtons}
        />
        <button className="default-button small-button" onClick={handleSave}>
          Save changes
        </button>
      </section>
      <section className="container left-aligned">
        <h1>Invite users</h1>
        <p>
          Add users by writing their e-mail in the field below. Alternatively,
          you can upload a comma separated file (CSV) with multiple e-mails.
          They will then get an invitation to create a user with that e-mail
          address.
        </p>
        <form onSubmit={handleSubmitSingleEmail}>
          <label style={{ display: "inline-block" }}>
            E-mail
            <input
              ref={singleEmail}
              type="email"
              name="email"
              placeholder="john.doe@company.com"
            ></input>
          </label>

          <button
            style={{ display: "inline-block" }}
            className="default-button small-button"
            type="submit"
          >
            Invite
          </button>
        </form>
        <form className="m-t-1" onSubmit={handleSubmitCsvEmail}>
          <label style={{ display: "inline-block" }}>
            Choose a file
            <input ref={csvEmail} type="file" accept=".csv"></input>
          </label>
          <button
            style={{ display: "inline-block" }}
            className="default-button small-button"
            type="submit"
          >
            Invite
          </button>
        </form>
      </section>
    </>
  );
}
