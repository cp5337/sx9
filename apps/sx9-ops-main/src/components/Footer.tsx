import React from 'react';

const Footer: React.FC = () => {
  return (
    <footer className="bg-gray-800 text-center py-6">
      <div className="container mx-auto px-4">
        <p className="text-sm text-gray-400">&copy; 2023 SIXGEN. All rights reserved.</p>
        <div className="mt-2">
          <a href="#" className="text-gray-400 hover:text-white mx-2">Privacy Policy</a>
          <a href="#" className="text-gray-400 hover:text-white mx-2">Terms of Service</a>
          <a href="#" className="text-gray-400 hover:text-white mx-2">Contact</a>
        </div>
      </div>
    </footer>
  );
};

export default Footer;