

const { Datastore } = require('notarealdb');

const store = new Datastore('./Data');

module.exports = {
    students: store.collection('students'),
    colleges: store.collection('colleges')
};

