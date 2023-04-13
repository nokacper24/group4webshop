import { useState, useRef } from "react";
import { NavLink } from "react-router-dom";

/**
 * Represents the header and navbar.
 * Contains ProFlex' logo, a search form, and links to navigate the website.
 * When the hamburger icon is clicked, toggle between open and closed menu.
 *
 * @returns A Header component.
 */
export default function Header() {
  const navToggle = useRef<HTMLButtonElement>(null);
  const navList = useRef<HTMLUListElement>(null);
  const nav = useRef<HTMLElement>(null);

  const [isOpen, setIsOpen] = useState(false);
  const toggleOpen = () => {
    setIsOpen((s) => !s);
  };

  const openMenu = () => {
    navList.current?.setAttribute("aria-expanded", "true");
    navList.current?.setAttribute("data-state", "open");
    if (nav.current) {
      nav.current.style.height = "100vh";
      nav.current.style.width = "100%";
    }
  };

  const closeMenu = () => {
    navList.current?.setAttribute("aria-expanded", "false");
    navList.current?.setAttribute("data-state", "closing");

    navList.current?.addEventListener(
      "animationend",
      () => {
        navList.current?.setAttribute("data-state", "closed");

        if (nav.current) {
          nav.current.style.height = "var(--header-height)";
          nav.current.style.width =
            "calc(var(--header-height) + var(--scrollbar-width))";
        }
      },
      { once: true }
    );
  };

  const setNavPositionProperty = () => {
    if (nav.current) {
      nav.current.style.position = isOpen ? "absolute" : "fixed";
    }
  };

  const handleNavClick = () => {
    navToggle.current?.classList.remove("active");

    closeMenu();
    setIsOpen(false);

    if (nav.current) {
      nav.current.style.position = "absolute";
    }
  };

  const handleClick = () => {
    // Toggle icon from hamburger to X
    navToggle.current?.classList.toggle("active");

    // Toggle between open and close menu
    isOpen ? closeMenu() : openMenu();
    toggleOpen();

    // Set the nav position to be absolute when the menu
    // is closed, and fixed when the menu is open
    setNavPositionProperty();
  };

  return (
    <header>
      <div className="header-container">
        <a href="#main" id="skip-navigation">
          Skip Navigation
        </a>

        {/* ProFlex Logo */}
        <NavLink onClick={handleNavClick} className="logo-link" to="/">
          <svg
            className="logo"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 878 299"
          >
            <title>ProFlex</title>
            <desc>Opens the home page</desc>
            <path d="m20.6 187.8l22.3-143.1h42.4q18.9 0 29.8 7.3 5.2 3.5 8.3 9.2 6 11 3.7 26.1-7 44.7-56.3 44.7h-27l-8.7 55.8zm61.2-129.3h-26.5l-9.4 59.9h24.1q18.8 0 28.3-6.6 10.2-6.9 12.9-24.3 2.5-15.7-6.8-23-7.4-6-22.6-6zm53.8 129.3l22.3-143.1h42.5q19.6 0 29.7 8.1 13.4 10.7 10.2 30.8-2.8 18.1-16.4 27.8-8 5.7-23.9 9l-0.1 0.6q11.4 4.6 17.3 18.9 6.9 16.7 15.7 47.9h-17.6q-8.4-36.9-15.8-49.2-8-13-24.5-13h-15.3l-9.8 62.2zm60.3-129.4h-25.7l-8.4 54h23.1q16.2 0 26.5-6.7 11.2-7.1 13.4-21.4 4-25.9-28.9-25.9zm132.9-16.4q17.8 0 30.3 10.5 13.7 11.4 17.6 32.3 2.6 14.4-0.1 31.4-3.5 22.6-14.5 40.2-11.1 17.8-27.6 26.8-13.8 7.5-29.3 7.5-21 0-34.1-13.6-12-12.2-14.6-32.8-1.7-13.1 0.6-28.1 5.6-35.5 28.3-56.5 19.1-17.7 43.4-17.7zm-2.7 13.9q-14.3 0-26.8 10-21.1 16.6-26.4 50.4-2.9 19.1 1.2 33.2 3.3 11.3 10.6 18.1 9.2 8.6 22.9 8.6 20.4 0 35.6-18.1 13.6-16.2 17.6-41.8 4.1-26.3-5-42.9-9.8-17.5-29.7-17.5z" />
            <path d="m398.7 257.4l21.5-121.9h83.5l-4 22.6h-44.9l-4.6 26h39.4l-4.4 24.9h-39.3l-8.6 48.4zm103.5 0l21.5-121.9h38.6l-17.1 97.1h41.4l-4.4 24.8zm99.3 0l21.5-121.9h85.6l-4 22.6h-46.9l-4.6 25.8h41.4l-3.9 22h-41.4l-4.7 26.7h47l-4.4 24.8zm242.4-121.9l-41.1 61.2 19.6 60.7h-46.9l-7.4-45.9-29 45.9h-35.4l41.1-60.7-19.6-61.2h47l11 43.2 27.6-43.2z" />
          </svg>
        </NavLink>

        <nav ref={nav}>
          {/* Hamburger menu toggle */}
          <div className="toggle-container">
            <button
              ref={navToggle}
              id="nav-toggle"
              aria-label="Menu"
              aria-controls="primary-navigation"
              onClick={handleClick}
            >
              <span className="hamburger" aria-hidden="true"></span>
            </button>
          </div>

          <ul
            ref={navList}
            className="nav-list"
            id="primary-navigation"
            data-state="closed"
          >
            <li>
              <NavLink onClick={handleNavClick} className="nav-link" to="/">
                Home
              </NavLink>
            </li>
            <li>
              <NavLink
                onClick={handleNavClick}
                className="nav-link"
                to="/products"
              >
                Products
              </NavLink>
            </li>
            <li>
              <NavLink
                onClick={handleNavClick}
                className="nav-link"
                to="/about"
              >
                About
              </NavLink>
            </li>
            <li>
              <NavLink
                onClick={handleNavClick}
                className="nav-link"
                to="/support"
              >
                Support
              </NavLink>
            </li>
            {/* User profile icon */}
            <li>
              <NavLink
                onClick={handleNavClick}
                className="nav-link"
                to="/profile"
              >
                <svg
                  className="nav-icon"
                  xmlns="http://www.w3.org/2000/svg"
                  viewBox="0 0 448 512"
                >
                  {/* Font Awesome Pro 6.2.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2022 Fonticons, Inc. */}
                  <title>User Profile</title>
                  <desc>Opens the user profile page</desc>
                  <path d="M224 256c70.7 0 128-57.3 128-128S294.7 0 224 0S96 57.3 96 128s57.3 128 128 128zm-45.7 48C79.8 304 0 383.8 0 482.3C0 498.7 13.3 512 29.7 512H418.3c16.4 0 29.7-13.3 29.7-29.7C448 383.8 368.2 304 269.7 304H178.3z" />
                </svg>
                <span className="icon-text">Profile</span>
              </NavLink>
            </li>
          </ul>
        </nav>
      </div>
    </header>
  );
}
