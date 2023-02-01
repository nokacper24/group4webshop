import { useState } from 'react';
import './App.css';
import { BrowserRouter, Routes, Route } from "react-router-dom";
import Navbar from './Components/Navbar'
import Footer from './Components/Footer';
import Home from './Components/home/Home';
import Products from './Components/products/Products';
import About from './Components/About';
import Support from './Components/Support';
import Profile from './Components/Profile';

function App() {
    const [count, setCount] = useState(0);

    return (
        <BrowserRouter>
            <Navbar />

            <main id="main">
                <Routes>
                    <Route path="/" element={<Home/>} />
                    <Route path="products" element={<Products/>} />
                    <Route path="about" element={<About/>} />
                    <Route path="support" element={<Support/>} />
                    <Route path="profile" element={<Profile/>} />
                </Routes>
            </main>

            <Footer />
        </BrowserRouter>

    )
}

export default App;
