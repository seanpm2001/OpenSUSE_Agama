import React from 'react';

export default (props) => {
  return (
    // Simple SVG square based on a wikimedia example https://commons.wikimedia.org/wiki/SVG_examples
    <svg xmlns="http://www.w3.org/2000/svg" version="1.1" width="48" height="48" {...props}>
      <rect x="0" y="0" width="48" height="48" />
    </svg>
  );
}
