import { Component } from "react";
import { Routes, Route } from "react-router-dom";
import { CreateAccountRegister } from "./CreateAccount_register";
import { CreateAccountVerify } from "./CreateAccount_verify";
import CreateAccountDetails from "./CreateAccount_details";

export default class CreateAccount extends Component {
  render() {
    return (
      <div className="center-container">
        <form className="container form-container">
          <h1>Create new account</h1>

          <Routes>
            <Route path="/" element={<CreateAccountRegister />}></Route>
            <Route path="verify" element={<CreateAccountVerify />}></Route>
            <Route path="details" element={<CreateAccountDetails />}></Route>
          </Routes>
        </form>
      </div>
    );
  }
}
