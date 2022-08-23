const useESModules = !!process.env.MODULE;

module.exports = () => {
  return {
    plugins: [
      ['@babel/transform-runtime', { useESModules }],
      '@babel/proposal-object-rest-spread',
      '@babel/proposal-class-properties',
      '@babel/proposal-export-default-from',
      'macros',
    ],
    presets: useESModules ? ['@babel/react'] : ['@babel/env', '@babel/react']
  };
};
