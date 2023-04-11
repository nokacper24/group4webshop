import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import { User } from "../../Interfaces";

interface PartialUser {
  email?: string;
}

export default function EditProfile() {
  let baseUrl = import.meta.env.VITE_URL + ":" + import.meta.env.VITE_PORT;
  // Check if we are in production mode
  if (import.meta.env.PROD) {
    baseUrl = "";
  }

  const navigate = useNavigate();

  const { userId } = useParams();
  const [user, setUser] = useState<User>();

  const [email, setEmail] = useState<string>("");

  const fetchUser = async () => {
    const response = await fetch(`${baseUrl}/api/priv/users/${userId}`);
    const data = await response.json();
    const user: User = data;
    setUser(user);
    return user;
  };

  /* TODO: Do a single PATCH request for user  */
  /* Include object property onlt if value is passed as parameter */

  const patchUser = async (email?: string) => {
    let user: PartialUser = {};
    if (email) {
      user.email = email;
    }

    fetch(`${baseUrl}/api/priv/users/${userId}`, {
      method: "PATCH",
      headers: {
        Accept: "application:json",
        "Content-Type": "application/json",
      },
      body: JSON.stringify(user),
    }).then((response) => {
      if (response.status == 200) {
        // Refresh
        navigate(0);
      } else {
        alert("Failed to save changes");
      }
    });
  };

  const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    if (email != user?.email) {
      patchUser(email);
    }
  };

  const handlePasswordReset = () => {
    fetch(`${baseUrl}/api/reset_password`, {
      method: "POST",
      headers: {
        Accept: "application/json",
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        email: user?.email,
      }),
    }).then((response) => {
      let status = response.status;

      if (status == 200) {
        alert(
          "We have sent you an e-mail with a link to reset password. It may take a few minutes. Check the spam folder if you do not see it."
        );
      } else {
        alert("Failed to reset password.");
      }
    });
  };

  useEffect(() => {
    fetchUser()
      .then((user: User) => {
        setUser(user);
        setEmail(user.email);
      })
      .catch(() => alert("Failed to load user details"));
  }, []);

  return (
    <>
      <section className="container left-aligned">
        <h1>Edit Profile</h1>
        <form onSubmit={(event) => handleSubmit(event)}>
          {/* Prevent implicit submission of the form */}
          <button
            type="submit"
            disabled
            style={{ display: "none" }}
            aria-hidden="true"
          ></button>

          <label>
            E-mail
            <input
              type="text"
              style={{ width: "min(30em, 70vw)" }}
              placeholder="E-mail"
              defaultValue={user?.email}
              onChange={(event) => setEmail(event.target.value)}
            ></input>
          </label>

          <div className="button-container">
            <button type="submit" className="default-button small-button">
              Save
            </button>
          </div>
        </form>
      </section>
      <section className="container left-aligned">
        <h1>Change password</h1>
        <button
          className="default-button small-button"
          onClick={handlePasswordReset}
        >
          Reset password
        </button>
      </section>
    </>
  );
}
