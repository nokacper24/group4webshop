import { Link } from 'react-router-dom';

function Navbar() {
    return (
        <nav className="navbar">
            <div className="logo">
                <img src="something" />
                {/* Searchbox */}
                <input type="text" placeholder="Search" />
            </div>

            <div className="links">
                <Link to="/">Home</Link>
                <Link to="/products">Products</Link>
                <Link to="/about">About</Link>
                <Link to="/support">Support</Link>
                {/* User profile logo */}
                <Link to="/profile"><img src="something" /> </Link>
            </div>
        </nav>
    );
}

export default Navbar;