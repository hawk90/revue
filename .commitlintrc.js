module.exports = {
  extends: ['@commitlint/config-conventional'],
  rules: {
    'subject-case': [2, 'never', ['sentence-case', 'start-case', 'pascal-case', 'upper-case']],
    'body-max-line-length': [0, 'always', 100],
  },
  ignores: [
    (message) => message.includes('GitHub'),
    (message) => message.includes('README'),
    (message) => message.includes('CONTRIBUTING'),
    (message) => /^Merge/.test(message),
  ],
};
