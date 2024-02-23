import '@testing-library/jest-dom';
import ResizeObserver from 'resize-observer-polyfill';

global.ResizeObserver = ResizeObserver; // see https://github.com/ZeeCoder/use-resize-observer/issues/40#issuecomment-1540994404
