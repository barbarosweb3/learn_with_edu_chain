// src/Dashboard.js
import React, { useState, useEffect } from 'react';

const CourseCard = ({ course }) => (
    <div className="bg-white rounded-lg shadow-md p-6 hover:shadow-lg transition-shadow">
        <h3 className="text-xl font-bold mb-2">{course.title}</h3>
        <p className="text-gray-600 mb-4">{course.metadata}</p>
        <div className="flex justify-between items-center">
      <span className="text-blue-600 font-semibold">
        ${course.price}
      </span>
            <button className="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600">
                Enroll Now
            </button>
        </div>
    </div>
);

const CreateCourseModal = ({ isOpen, onClose, onSubmit }) => {
    const [formData, setFormData] = useState({
        title: '',
        metadata: '',
        price: 0
    });

    if (!isOpen) return null;

    const handleSubmit = (e) => {
        e.preventDefault();
        onSubmit(formData);
        setFormData({ title: '', metadata: '', price: 0 });
    };

    return (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
            <div className="bg-white p-8 rounded-lg w-full max-w-md">
                <h2 className="text-2xl font-bold mb-4">Create New Course</h2>
                <form onSubmit={handleSubmit}>
                    <div className="mb-4">
                        <label className="block text-gray-700 text-sm font-bold mb-2">
                            Title
                        </label>
                        <input
                            type="text"
                            className="w-full p-2 border rounded focus:outline-none focus:border-blue-500"
                            value={formData.title}
                            onChange={(e) => setFormData({...formData, title: e.target.value})}
                            required
                        />
                    </div>
                    <div className="mb-4">
                        <label className="block text-gray-700 text-sm font-bold mb-2">
                            Description
                        </label>
                        <textarea
                            className="w-full p-2 border rounded focus:outline-none focus:border-blue-500"
                            value={formData.metadata}
                            onChange={(e) => setFormData({...formData, metadata: e.target.value})}
                            required
                            rows="4"
                        />
                    </div>
                    <div className="mb-6">
                        <label className="block text-gray-700 text-sm font-bold mb-2">
                            Price
                        </label>
                        <input
                            type="number"
                            className="w-full p-2 border rounded focus:outline-none focus:border-blue-500"
                            value={formData.price}
                            onChange={(e) => setFormData({...formData, price: Number(e.target.value)})}
                            required
                            min="0"
                            step="0.01"
                        />
                    </div>
                    <div className="flex justify-end gap-2">
                        <button
                            type="button"
                            onClick={onClose}
                            className="px-4 py-2 text-gray-600 hover:bg-gray-100 rounded transition-colors"
                        >
                            Cancel
                        </button>
                        <button
                            type="submit"
                            className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors"
                        >
                            Create Course
                        </button>
                    </div>
                </form>
            </div>
        </div>
    );
};

const Dashboard = () => {
    const [courses, setCourses] = useState([]);
    const [isModalOpen, setIsModalOpen] = useState(false);
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        fetchCourses();
    }, []);

    const fetchCourses = async () => {
        try {
            setLoading(true);
            const response = await fetch('http://localhost:8080/api/v1/courses');
            const data = await response.json();
            setCourses(data.data || []);
        } catch (error) {
            console.error('Error fetching courses:', error);
        } finally {
            setLoading(false);
        }
    };

    const handleCreateCourse = async (courseData) => {
        try {
            const response = await fetch('http://localhost:8080/api/v1/courses', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(courseData),
            });

            if (!response.ok) {
                throw new Error('Network response was not ok');
            }

            const data = await response.json();
            if (data.success) {
                await fetchCourses();
                setIsModalOpen(false);
            }
        } catch (error) {
            console.error('Error creating course:', error);
            alert('Failed to create course. Please try again.');
        }
    };

    return (
        <div className="min-h-screen bg-gray-100 flex flex-col">
            <nav className="bg-white shadow-sm">
                <div className="max-w-7xl mx-auto px-4 py-4">
                    <div className="flex justify-between items-center">
                        <div className="flex items-center space-x-2">
                            <span className="text-xl font-bold">OpenCampus Learn</span>
                        </div>
                        <button
                            onClick={() => setIsModalOpen(true)}
                            className="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600 transition-colors"
                        >
                            Create Course
                        </button>
                    </div>
                </div>
            </nav>

            <main className="flex-grow max-w-7xl mx-auto px-4 py-8">
                <div className="mb-8">
                    <h1 className="text-3xl font-bold text-gray-900">Your Learning Dashboard</h1>
                    <p className="mt-2 text-gray-600">Manage your courses and certificates</p>
                </div>

                {loading ? (
                    <div className="text-center py-12">
                        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500 mx-auto"></div>
                        <p className="mt-4 text-gray-600">Loading courses...</p>
                    </div>
                ) : courses.length === 0 ? (
                    <div className="text-center py-12">
                        <p className="text-gray-600">No courses available yet.</p>
                        <button
                            onClick={() => setIsModalOpen(true)}
                            className="mt-4 text-blue-500 hover:text-blue-600"
                        >
                            Create your first course
                        </button>
                    </div>
                ) : (
                    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                        {courses.map((course) => (
                            <CourseCard key={course.id} course={course} />
                        ))}
                    </div>
                )}

                <CreateCourseModal
                    isOpen={isModalOpen}
                    onClose={() => setIsModalOpen(false)}
                    onSubmit={handleCreateCourse}
                />
            </main>

            <footer className="bg-white border-t mt-auto">
                <div className="max-w-7xl mx-auto px-4 py-6">
                    <p className="text-center text-gray-600">
                        © 2025 OpenCampus Learn. All rights reserved.
                    </p>
                </div>
            </footer>
        </div>
    );
};

export default Dashboard;