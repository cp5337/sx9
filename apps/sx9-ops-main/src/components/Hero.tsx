import React from 'react';

const Hero: React.FC = () => {
  return (
    <div className="relative bg-gray-900 overflow-hidden py-12 sm:py-16">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="relative z-10 text-center sm:text-left">
          <h1 className="text-2xl sm:text-3xl md:text-4xl font-light text-white tracking-tight leading-tight">
            <span className="block mb-1">Digital Dominance</span>
            <span className="block text-blue-400">Against Global Threats</span>
          </h1>
          <p className="mt-3 text-sm sm:text-base md:text-lg text-gray-300 max-w-2xl mx-auto sm:mx-0">
            Next Generation Cybersecurity Platform
          </p>
          <div className="mt-6 sm:flex sm:justify-center lg:justify-start">
            <div className="rounded-md shadow">
              <a href="#" className="w-full flex items-center justify-center px-6 py-2 border border-transparent text-base font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 transition duration-150 ease-in-out">
                Get started
              </a>
            </div>
            <div className="mt-3 sm:mt-0 sm:ml-3">
              <a href="#" className="w-full flex items-center justify-center px-6 py-2 border border-transparent text-base font-medium rounded-md text-blue-600 bg-gray-800 hover:bg-gray-700 transition duration-150 ease-in-out">
                Learn more
              </a>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Hero;