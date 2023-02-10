import { useState, useEffect } from "react";
import { Routes, Route } from "react-router-dom";
import MyAccount from "./MyAccount";
import SignIn from "./SignIn";

export default function Profile() {
  const [authenticated, setAuthenticated] = useState<string | null>(null);

  useEffect(() => {
    const loggedInUser: string | null = localStorage.getItem("authenticated");
    if (loggedInUser) {
      setAuthenticated(loggedInUser);
    }
  }, []);

  if (!authenticated) {
    return <SignIn />;
  } else {
    return <MyAccount />;
  }

  /* return (
    <Routes>
      <Route path="sign-in" element={<SignIn />}></Route>
      <Route path="my-account" element={<MyAccount />} />
    </Routes>
  ); */
}
