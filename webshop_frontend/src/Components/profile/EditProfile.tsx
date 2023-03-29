import { useEffect, useRef, useState } from "react";
import { useParams } from "react-router-dom";
import { User } from "../../Interfaces";

export default function EditProfile() {
  let baseUrl = import.meta.env.VITE_URL + ":" + import.meta.env.VITE_PORT;
  // Check if we are in production mode
  if (import.meta.env.PROD) {
    baseUrl = "";
  }

  const { userId } = useParams();
  const [user, setUser] = useState<User>();

  const [email, setEmail] = useState<string>("");
  const [oldPassword, setOldPassword] = useState<string>("");
  const [newPassword, setNewPassword] = useState<string>("");
  const [confirmPassword, setConfirmPassword] = useState<string>("");
  const [formAlert, setFormAlert] = useState<string>("");

  const fetchUser = async () => {
    const response = await fetch(`${baseUrl}/api/users/${userId}`);
    const data = await response.json();
    const user: User = data;
    setUser(user);
    return user;
  };

  const patchEmail = async (email: string) => {
    fetch(`${baseUrl}/api/users/${userId}`, {
      method: "PATCH",
      headers: {
        Accept: "application/json",
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        email: email,
      }),
    })
      .then((response) => {
        const status = response.status;
        if (status == 200) {
          location.reload();
        } else {
          alert("Something went wrong when saving e-mail");
        }
      })
      .catch(() => alert("Failed to save e-mail"));
  };

  const patchPassword = async (newPassword: string) => {
    fetch(`${baseUrl}/api/users/${userId}`, {
      method: "PATCH",
      headers: {
        Accept: "application/json",
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        password: newPassword,
      }),
    })
      .then((response) => {
        const status = response.status;
        if (status == 200) {
          location.reload();
        } else {
          alert("Something went wrong when saving password");
        }
      })
      .catch(() => alert("Failed to save password"));
  };

  const checkCorrectPassword = async (oldPassword: string) => {
    let result = await fetch(`${baseUrl}/api/confirm_password`, {
      method: "GET",
      headers: {
        Accept: "application/json",
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        user_id: userId,
        password: oldPassword,
      }),
    });

    if (result.status == 200) {
      return true;
    } else {
      return false;
    }
  };

  const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    // Check if confirmed password is incorrect
    if (newPassword != confirmPassword) {
      setFormAlert(
        "Confirmed password does not match new password. Please make sure the passwords are identical"
      );
    } else {
      // Check if user wants to update password
      if (newPassword != "") {
        // Check if old password is correct
        checkCorrectPassword(oldPassword).then((result) => {
          if (result) {
            setFormAlert("");
            patchPassword(newPassword);
          } else {
            setFormAlert("Old password is not correct");
          }
        });
      }
      // Check if user wants to change e-mail
      if (email != user?.email) {
        patchEmail(email);
      }
    }
  };

  useEffect(() => {
    fetchUser()
      .then((user: User) => setUser(user))
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
              placeholder="E-mail"
              defaultValue={user?.email}
              onChange={(event) => setEmail(event.target.value)}
            ></input>
          </label>
          <label>
            Old password
            <input
              type="password"
              placeholder="Old password"
              onChange={(event) => setOldPassword(event.target.value)}
            ></input>
          </label>
          <label>
            New password
            <input
              type="password"
              placeholder="New password"
              onChange={(event) => setNewPassword(event.target.value)}
            ></input>
          </label>
          <label>
            Confirm new password
            <input
              type="password"
              placeholder="Confirm new password"
              onChange={(event) => setConfirmPassword(event.target.value)}
            ></input>
          </label>
          <div className="button-container">
            <button type="submit" className="default-button small-button">
              Save
            </button>
            <p className="form-alert">{formAlert}</p>
          </div>
        </form>
      </section>
    </>
  );
}
