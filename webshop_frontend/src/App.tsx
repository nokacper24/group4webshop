import { useState } from "react";
import "./App.css";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import Header from "./Components/Header";
import Footer from "./Components/Footer";
import Home from "./Components/home/Home";
import Products from "./Components/products/Products";
import ProductPage from "./Components/product-page/ProductPage";
import PurchaseLicense from "./Components/product-page/PurchaseLicense";
import About from "./Components/about-us/About";
import Support from "./Components/support/Support";

function App() {
  const [count, setCount] = useState(0);
import Profile from "./Components/profile/Profile";

  return (
    <BrowserRouter>
      <Header />

      <main id="main">
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="products" element={<Products />} />
          <Route path="about" element={<About />} />
          <Route path="support" element={<Support />} />
          <Route path="profile" element={<Profile />} />
          <Route path="product-page" element={<ProductPage />} />
          <Route path="purchase-license" element={<PurchaseLicense />} />
        </Routes>
      </main>

      <Footer />
    </BrowserRouter>
  );
}

export default App;
