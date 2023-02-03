import { useState } from 'react';
import './App.css';
import { BrowserRouter, Routes, Route } from "react-router-dom";
import Header from './Components/Header';
import Footer from './Components/Footer';
import Home from './Components/home/Home';
import Products from './Components/products/Products';
import About from './Components/About';
import Support from './Components/Support';
import Profile from './Components/Profile';
import Product_page from './Components/product-page/ProductPage';

function App() {
    const [count, setCount] = useState(0);

    return (
        <BrowserRouter>
            <Header />

            <main id="main">
                <Routes>
                    <Route path="/" element={<Home/>} />
                    <Route path="products" element={<Products/>} />
                    <Route path="about" element={<About/>} />
                    <Route path="support" element={<Support/>} />
                    <Route path="profile" element={<Profile/>} />
                    <Route path="product-page" element={<Product_page/>} />
                </Routes>
            </main>

            <Footer />
        </BrowserRouter>

    )
}

export default App;
