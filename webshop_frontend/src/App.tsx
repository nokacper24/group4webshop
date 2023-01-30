import { useState } from 'react'
import reactLogo from './assets/react.svg'
import './App.css'
import { BrowserRouter, Routes, Route } from "react-router-dom";
import Navbar from './Components/Navbar'
import Footer from './Components/Footer';
import Home from './Components/Home';
import Products from './Components/Products';
import About from './Components/About';
import Support from './Components/Support';
import Profile from './Components/Profile';

function App() {
    const [count, setCount] = useState(0)

    return (
        <div className="App">
            <BrowserRouter>
                <nav className="navbar">
                    <Navbar />
                </nav>
                <Routes>
                    <Route path="/" element={<Home/>} />
                        <Route path="products" element={<Products/>} />
                        <Route path="about" element={<About/>} />
                        <Route path="support" element={<Support/>} />
                        <Route path="profile" element={<Profile/>} />
                </Routes>
                <Footer />
            </BrowserRouter>
        </div>
    )
}

export default App
