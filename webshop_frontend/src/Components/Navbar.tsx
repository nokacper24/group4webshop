import { Link } from 'react-router-dom';
import logo from '../assets/ProFlex.svg';
import avatar from '../assets/avatar.svg';

function Navbar() {
    return (
        <nav>
            <div className="logo">
                <img className="nav-icon" src={logo} />
                {/* Searchbox */}
                <input type="text" placeholder="Search" />
            </div>

            <div className="nav-links">
                <Link className="nav-link" to="/">Home</Link>
                <Link className="nav-link" to="/products">Products</Link>
                <Link className="nav-link" to="/about">About</Link>
                <Link className="nav-link" to="/support">Support</Link>
                {/* User profile logo */}
                <Link className="nav-link" to="/profile"><img className="nav-icon" src={avatar} /> </Link>
            </div>
        </nav>
    );
}

export default Navbar;